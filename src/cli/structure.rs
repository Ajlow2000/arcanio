use clap::{arg, command, Parser, Subcommand, ArgAction};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,

    #[arg(short, long, action = ArgAction::Count, global = true)]
    pub verbose: u8,

}

#[derive(Subcommand, Debug)]
pub enum Command {
    Test,
}
