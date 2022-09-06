use std::path::PathBuf;
use std::str::FromStr;

use clap::Parser;
use tokio::runtime::Runtime;

use crate::common::{agent::anonymous_agent, canisters::governance};
use crate::types::governance::{ListNeurons, ListNeuronsResponse, NeuronInfo};
use crate::utils::waiter::waiter;

/// The governance canister method to be accessed for this command.
const METHOD_NAME: &str = "list_neurons";

/// The header contents for CSV output.
const CSV_HEADER: &[&str] = &[
    "Neuron ID",
    "Age in Seconds",
    "Created Timestamp Seconds",
    "Dissolve Delay in Seconds",
    "Staked e8s",
    "State",
    "Voting Power",
];

/// The type of output that should be given.
#[derive(Debug)]
pub enum Output {
    /// Pretty-prints the response.
    PrettyPrint,
    /// Outputs the response as a CSV file.
    Csv,
    /// Outputs the response as JSON.
    Json,
}

impl core::fmt::Display for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Output::PrettyPrint => write!(f, "pp"),
            Output::Csv => write!(f, "csv"),
            Output::Json => write!(f, "json"),
        }
    }
}

impl FromStr for Output {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pp" => Ok(Output::PrettyPrint),
            "csv" => Ok(Output::Csv),
            "json" => Ok(Output::Json),
            _ => Err(String::from("Invalid output option provided")),
        }
    }
}

#[derive(Debug, Parser)]
pub struct NeuronListOpts {
    /// A file containing a list of neuron IDs.
    #[clap(short = 'f', long)]
    pub argument_file: Option<String>,
    /// A list of neuron IDs (-n 1 -n 2 ... -n n).
    #[clap(short, long)]
    pub neuron_ids: Vec<u64>,
    /// Determines how results should be provided to the user: CSV (csv), JSON (json), or pretty-printed (pp).
    #[clap(short, long, default_value("json"))]
    pub output: Output,
}

pub fn exec(opts: NeuronListOpts) {
    let agent = anonymous_agent();
    let canister = governance::canister(&agent);
    let list_neurons_arg = build_arg(&opts);
    let runtime = Runtime::new().expect("Failed to create a runtime.");
    let mut response = runtime.block_on(async {
        canister
            .update_(METHOD_NAME)
            .with_arg(list_neurons_arg)
            .build::<(ListNeuronsResponse,)>()
            .call_and_wait(waiter())
            .await
            .expect("Failed to make call to IC.")
            .0
    });

    response
        .neuron_infos
        .sort_by(|(a_id, _), (b_id, _)| a_id.cmp(b_id));

    render(&opts, response);
}

fn build_arg(opts: &NeuronListOpts) -> ListNeurons {
    let mut arg = ListNeurons {
        neuron_ids: opts.neuron_ids.clone(),
        include_neurons_readable_by_caller: false,
    };

    if let Some(ref path_input) = opts.argument_file {
        let expanded_path = shellexpand::tilde(path_input);
        let path = PathBuf::from(expanded_path.to_string());
        let contents = std::fs::read_to_string(path).expect("Failed to read argument file!");
        let mut rows = contents
            .split('\n')
            .filter_map(|id| id.parse::<u64>().ok())
            .collect::<Vec<_>>();
        arg.neuron_ids.append(&mut rows);
    }

    arg
}

fn render(opts: &NeuronListOpts, response: ListNeuronsResponse) {
    match opts.output {
        Output::PrettyPrint => println!("{:#?}", response),
        Output::Csv => render_csv(response),
        Output::Json => println!(
            "{}",
            serde_json::to_string_pretty(&response).expect("Failed to create JSON from response!")
        ),
    }
}

fn render_csv(response: ListNeuronsResponse) {
    let stdout = std::io::stdout();
    let mut writer = csv::WriterBuilder::new().from_writer(stdout);
    writer
        .write_record(CSV_HEADER)
        .expect("Failed to write neuron info!");

    for (neuron_id, neuron_info) in response.neuron_infos {
        writer
            .write_record(&csv_row(neuron_id, neuron_info))
            .expect("Failed to write neuron info!");
    }
}

/// Layout a single row for CSV output.
fn csv_row(neuron_id: u64, neuron_info: NeuronInfo) -> Vec<String> {
    vec![
        neuron_id.to_string(),
        neuron_info.age_seconds.to_string(),
        neuron_info.created_timestamp_seconds.to_string(),
        neuron_info.dissolve_delay_seconds.to_string(),
        neuron_info.stake_e8s.to_string(),
        neuron_info.state.to_string(),
        neuron_info.voting_power.to_string(),
    ]
}
