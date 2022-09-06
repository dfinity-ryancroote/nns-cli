#![allow(non_camel_case_types, clippy::large_enum_variant)]
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
