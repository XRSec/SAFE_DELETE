use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::{ArgAction, Parser};

#[cfg(target_os = "macos")]
use trash::macos::{DeleteMethod, TrashContextExtMacos};

#[derive(Debug, Parser)]
#[command(
    name = "rmf",
    about = "Safe Alternative System rm",
    version,
    disable_help_subcommand = true
)]
struct Args {
    /// Accept directories, matching rm -r semantics. Directories are moved to trash as a whole.
    #[arg(short = 'r', long = "recursive", action = ArgAction::SetTrue)]
    recursive: bool,

    /// Ignore missing paths and skip interactive prompts.
    #[arg(short = 'f', long = "force", action = ArgAction::SetTrue)]
    force: bool,

    /// Ask before moving each path to trash.
    #[arg(short = 'i', long = "interactive", action = ArgAction::SetTrue)]
    interactive: bool,

    /// Print each successfully trashed path.
    #[arg(short = 'v', long = "verbose", action = ArgAction::SetTrue)]
    verbose: bool,

    /// Accepted for rm compatibility.
    #[arg(long = "one-file-system", action = ArgAction::SetTrue)]
    one_file_system: bool,

    /// Files or directories to move to the system trash.
    #[arg(value_name = "PATH")]
    paths: Vec<PathBuf>,
}

fn create_trash_context() -> trash::TrashContext {
    #[allow(unused_mut)]
    let mut trash_ctx = trash::TrashContext::default();
    #[cfg(target_os = "macos")]
    {
        trash_ctx.set_delete_method(DeleteMethod::NsFileManager);
    }
    trash_ctx
}

fn normalize_path(path: &Path) -> PathBuf {
    let s = path.to_string_lossy();
    if (s.ends_with('/') || s.ends_with('\\')) && path != Path::new("/") {
        let trimmed = s.trim_end_matches(['/', '\\']);
        PathBuf::from(if trimmed.is_empty() { "/" } else { trimmed })
    } else {
        path.to_path_buf()
    }
}

fn main() -> ExitCode {
    let args = Args::parse();

    if args.paths.is_empty() {
        if !args.force {
            eprintln!("rmf: missing operand");
            return ExitCode::FAILURE;
        }
        return ExitCode::SUCCESS;
    }

    let trash_ctx = create_trash_context();
    let mut failed = false;

    for raw_path in &args.paths {
        let path = normalize_path(raw_path);

        match path.symlink_metadata() {
            Ok(_) => {}
            Err(err) if err.kind() == io::ErrorKind::NotFound && args.force => continue,
            Err(err) if err.kind() == io::ErrorKind::NotFound => {
                eprintln!("rmf: 文件不存在: {}", raw_path.display());
                failed = true;
                continue;
            }
            Err(err) => {
                eprintln!("rmf: 无法访问 {}: {err}", raw_path.display());
                failed = true;
                continue;
            }
        }

        if args.interactive && !args.force && !confirm(&path) {
            continue;
        }

        if let Err(err) = trash_ctx.delete(&path) {
            eprintln!("rmf: 删除文件失败: {}: {err}", raw_path.display());
            failed = true;
            continue;
        }

        if args.verbose {
            println!("Success: {}", raw_path.display());
        }
    }

    if failed {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

fn confirm(path: &Path) -> bool {
    print!("确认删除文件: {} [y/N] ", path.display());
    if io::stdout().flush().is_err() {
        return false;
    }

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => matches!(input.trim(), "y" | "Y"),
        Err(err) => {
            eprintln!("rmf: 获取输入失败: {err}");
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_path() {
        assert_eq!(normalize_path(Path::new("link/")), PathBuf::from("link"));
        assert_eq!(normalize_path(Path::new("link///")), PathBuf::from("link"));
        assert_eq!(normalize_path(Path::new("dir/subdir/")), PathBuf::from("dir/subdir"));
        assert_eq!(normalize_path(Path::new("file")), PathBuf::from("file"));
        assert_eq!(normalize_path(Path::new("/")), PathBuf::from("/"));
        assert_eq!(normalize_path(Path::new("///")), PathBuf::from("/"));
    }

    #[test]
    #[cfg(unix)]
    fn test_delete_symlink_folder_and_broken_symlink() {
        use std::fs;
        use std::os::unix::fs::symlink;

        let base = std::env::temp_dir().join("rmf_unit_test_symlinks");
        let _ = fs::remove_dir_all(&base);
        fs::create_dir_all(&base).unwrap();

        let target_dir = base.join("target_dir");
        fs::create_dir_all(&target_dir).unwrap();
        fs::write(target_dir.join("data.txt"), "hello").unwrap();

        let link_dir = base.join("link_dir");
        symlink(&target_dir, &link_dir).unwrap();

        // 1. Delete symlink to folder with trailing slash
        let link_dir_slash = PathBuf::from(format!("{}/", link_dir.display()));
        let normalized = normalize_path(&link_dir_slash);
        assert_eq!(normalized, link_dir);
        assert!(normalized.symlink_metadata().is_ok());

        let ctx = create_trash_context();
        assert!(ctx.delete(&normalized).is_ok());

        // Target directory and its contents must still be intact!
        assert!(target_dir.exists());
        assert!(target_dir.join("data.txt").exists());
        // The symlink itself must be gone
        assert!(!link_dir.exists());
        assert!(link_dir.symlink_metadata().is_err());

        // 2. Delete broken symlink with trailing slash
        let broken_link = base.join("broken_link");
        symlink("nonexistent_path", &broken_link).unwrap();
        let broken_slash = PathBuf::from(format!("{}/", broken_link.display()));
        let normalized_broken = normalize_path(&broken_slash);
        assert!(normalized_broken.symlink_metadata().is_ok());
        assert!(ctx.delete(&normalized_broken).is_ok());
        assert!(normalized_broken.symlink_metadata().is_err());

        let _ = fs::remove_dir_all(&base);
    }
}
