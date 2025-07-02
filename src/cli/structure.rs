use clap::{arg, command, Parser, Subcommand, ArgAction};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,

    /// Enable logging with configurable levels ('-v' enables INFO level, '-vvv' enables max TRACE level)
    #[arg(short, action = ArgAction::Count, global = true)]
    pub verbose: u8,

}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// A general purpose command endpoint used for adhoc feature testing
    Test,
}
