package cmd

import (
	"os"

	"github.com/spf13/cobra"
)


var rootCmd = &cobra.Command{
	Use:   "media-utilities",
	Short: "",
    Long: "",
    Version: "", 
    Run: func(cmd *cobra.Command, args []string) {
        cmd.Help()
    },
}

func Execute() {
	err := rootCmd.Execute()
	if err != nil {
		os.Exit(1)
	}
}

func init() {
    // viper.SetConfigName("config")
    // viper.SetConfigType("toml")
    // viper.AddConfigPath("$XDG_CONFIG_HOME/repo-manager") 

    // if err := viper.ReadInConfig(); err != nil {
    //     if _, ok := err.(viper.ConfigFileNotFoundError); ok {
    //         // Config file not found; ignore error if desired
    //     } else {
    //         // Config file was found but another error was produced
    //         panic(fmt.Errorf("fatal error config file: %w", err))
    //     }
    // }
}
