package main

import (
	"fmt"
	"github.com/Bios-Marcel/wastebasket"
	log "github.com/sirupsen/logrus"
	"github.com/spf13/cobra"
	"os"
)

var (
	recursive   bool
	force       bool
	interactive bool
	verbose     bool
)

var versionData string

var rootCmd = &cobra.Command{
	Short:   "Safe Alternative System rm",
	Long:    "Safe Alternative System rm",
	Version: versionData,

	Run: func(cmd *cobra.Command, args []string) {
		for _, v := range args {
			if _, err := os.Stat(v); err != nil {
				if os.IsNotExist(err) {
					if !force {
						log.Errorf("文件不存在: %s", v)
					}
					continue
				} else {
					if !force {
						log.Errorf("文件不存在: %s", v)
					}
				}
				continue
			}
			if interactive {
				log.Infof("确认删除文件: %s", v)
				var input string
				if _, err := fmt.Scanln(&input); err != nil {
					log.Errorf("获取输入失败: %v", err)
					return
				}
				if input != "y" && input != "Y" {
					log.Infof("取消删除文件: %s", v)
					continue
				}
			}
			if err := wastebasket.Trash(v); err != nil {
				log.Errorf("删除文件失败: %v err:%v", v, err)
				return
			}
			if verbose {
				log.Infof("Success: %s", v)
			}
		}
	},
	Example: "rm -rfi [file1] [file2] [file3]",
}

func main() {
	var temp string
	rootCmd.Flags().BoolVarP(&recursive, "recursive", "r", false, "which removes directories, removing the contents recursively beforehand (so as not to leave files without a directory to reside in).")
	rootCmd.Flags().BoolVarP(&force, "force", "f", false, "which ignores non-existent files and overrides any confirmation prompts (effectively canceling -i), although it will not remove files from a directory if the directory is write-protected.")
	rootCmd.Flags().BoolVarP(&interactive, "interactive", "i", false, "which asks for every deletion to be confirmed.")
	rootCmd.Flags().BoolVarP(&verbose, "verbose", "v", false, "which explains what is being done.")
	rootCmd.Flags().StringVarP(&temp, "one-file-system", "", "one-file-system", "only removes files on the same file system as the argument, and will ignore mounted file systems.")
	if err := rootCmd.Execute(); err != nil {
		log.Errorf("Error: %v", err)
	}
}
