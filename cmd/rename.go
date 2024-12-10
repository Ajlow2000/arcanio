/*
Copyright © 2024 NAME HERE <EMAIL ADDRESS>
*/
package cmd

import (
	"github.com/Ajlow2000/media-utilities/app"
	"github.com/spf13/cobra"
	"github.com/spf13/viper"
)

var (
    debug = false
    repo = "";
    urlPrefix = "";
    destination = "";
)

// addCmd represents the add command
var addCmd = &cobra.Command{
    Use:   "rename INPUT_FILE",
    DisableFlagsInUseLine: true,
    Args: cobra.ExactArgs(1),
	Short: "Rename a file to a standardized format",
	Long: "Rename a file to a standardized format" +
    "" +
    "",
	Run: func(cmd *cobra.Command, args []string) {
        if len(args) != 1 {
            cmd.Help()
        } else {
            repo = args[0]
            if (urlPrefix == "") {
                urlPrefix = viper.GetString("urlPrefix")
            }

            if (destination == "") {
                destination = viper.GetString(("managedDir"))
            }
		    app.Rename()
        }
	},
}

func init() {
	rootCmd.AddCommand(addCmd)

    addCmd.Flags().StringVar(&urlPrefix, "urlPrefix", "", "The url prefix for the repo name (Ex: git@github:myusername/)")
    addCmd.Flags().StringVar(&destination, "destination", "", "Filepath to clone the specified repo into")

}
