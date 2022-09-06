use crate::commands;
use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: commands::Command,
}
