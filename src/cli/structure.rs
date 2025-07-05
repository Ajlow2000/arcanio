use clap::{arg, command, Parser, Subcommand, ArgAction};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,

    /// Enable logging with configurable levels
    ///
    /// The configurable levels are mapped to the following values.
    /// When running cli commands and a verbosity flag is omitted, 
    /// logging is disabled.
    ///     '-v'    ERROR and WARN level
    ///     '-vv'   INFO level
    ///     '-vvv'  DEBUG level
    ///     '-vvvv' TRACE level
    #[arg(short, action = ArgAction::Count, global = true, verbatim_doc_comment)]
    pub verbose: u8,

    /// Path to configuration file
    ///
    /// If not specified, the application will look for configuration files
    /// in the following order:
    ///     1. ./arcanio.toml (current directory)
    ///     2. ~/.config/arcanio/config.toml (user config)
    ///     3. /etc/arcanio/config.toml (system config)
    #[arg(short, long, global = true, verbatim_doc_comment)]
    pub config: Option<String>,

}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Configuration management commands
    Config {
        #[command(subcommand)]
        command: ConfigCommand,
    },

    /// A general purpose command endpoint used for adhoc feature testing
    Temp,
}

#[derive(Subcommand, Debug)]
pub enum ConfigCommand {
    /// Print the default configuration to stdout
    Default,
    /// Print the current configuration to stdout
    Current,
}
