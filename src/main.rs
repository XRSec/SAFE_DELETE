use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::{ArgAction, Parser};

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

fn main() -> ExitCode {
    let args = Args::parse();

    if args.paths.is_empty() {
        if !args.force {
            eprintln!("rmf: missing operand");
            return ExitCode::FAILURE;
        }
        return ExitCode::SUCCESS;
    }

    let mut failed = false;

    for path in &args.paths {
        match path.symlink_metadata() {
            Ok(_) => {}
            Err(err) if err.kind() == io::ErrorKind::NotFound && args.force => continue,
            Err(err) if err.kind() == io::ErrorKind::NotFound => {
                eprintln!("rmf: 文件不存在: {}", path.display());
                failed = true;
                continue;
            }
            Err(err) => {
                eprintln!("rmf: 无法访问 {}: {err}", path.display());
                failed = true;
                continue;
            }
        }

        if args.interactive && !args.force && !confirm(path) {
            continue;
        }

        if let Err(err) = trash::delete(path) {
            eprintln!("rmf: 删除文件失败: {}: {err}", path.display());
            failed = true;
            continue;
        }

        if args.verbose {
            println!("Success: {}", path.display());
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
