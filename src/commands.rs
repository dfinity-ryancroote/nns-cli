use clap::Subcommand;

pub mod neuron;

#[derive(Subcommand)]
pub enum Command {
    Neuron(neuron::NeuronOpts)
}

pub fn dispatch(command: Command) {
    match command {
        Command::Neuron(o) => neuron::dispatch(o)
    }
}