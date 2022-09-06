use clap::Parser;
use crate::commands;

#[derive(Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: commands::Command
}