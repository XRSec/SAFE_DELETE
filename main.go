package main

import (
	"github.com/Bios-Marcel/wastebasket"
	log "github.com/sirupsen/logrus"
	"github.com/spf13/cobra"
	"os"
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
					log.Errorf("文件不存在: %s", v)
					continue
				}
				log.Infof("文件不存在? : %s", v)
				continue
			}

			if err := wastebasket.Trash(v); err != nil {
				log.Errorf("Error: %v", err)
				return
			}
			log.Infof("Success: %v", v)
		}
	},
	Example: "rm -rfi [file1] [file2] [file3]",
}

func main() {
	var temp string
	rootCmd.Flags().StringVarP(&temp, "recursive", "r", "recursive", "which removes directories, removing the contents recursively beforehand (so as not to leave files without a directory to reside in).")
	rootCmd.Flags().StringVarP(&temp, "force", "f", "force", "which ignores non-existent files and overrides any confirmation prompts (effectively canceling -i), although it will not remove files from a directory if the directory is write-protected.")
	rootCmd.Flags().StringVarP(&temp, "interactive", "i", "interactive", "which asks for every deletion to be confirmed.")
	rootCmd.Flags().StringVarP(&temp, "directory", "d", "directory", "which deletes an empty directory, and only works if the specified directory is empty.")
	rootCmd.Flags().StringVarP(&temp, "one-file-system", "", "one-file-system", "only removes files on the same file system as the argument, and will ignore mounted file systems.")
	if err := rootCmd.Execute(); err != nil {
		log.Errorf("Error: %v", err)
	}
}
