mod cli;
mod commands;
mod common;
mod types;
mod utils;

use crate::cli::Cli;
use clap::Parser;

fn main() {
    let cli = Cli::parse();
    commands::dispatch(cli.command)
}
