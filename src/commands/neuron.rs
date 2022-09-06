mod list;

use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct NeuronOpts {
    #[clap(subcommand)]
    pub subcommand: Command
}

#[derive(Subcommand)]
pub enum Command {
    List(list::NeuronListOpts),
}

pub fn dispatch(opts: NeuronOpts) {
    match opts.subcommand {
        Command::List(o) => list::exec(o)
    }
}