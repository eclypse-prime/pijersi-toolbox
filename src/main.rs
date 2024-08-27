use clap::Parser;

use pijersi_rs::{board::Board, logic::translate::action_to_string};
use pijersi_toolbox::{actions::{backtrack_responses, get_positions, get_responses_at_depth}, args::{Cli, Mode}, io::{export_positions, export_responses, import_positions, import_responses}};

fn main() {
    let args = Cli::parse();

    let output_path = args.output;
    let exploration_depth = args.exploration_depth;
    let search_depth = args.search_depth;


    match args.mode {
        Mode::New => {
            let positions = get_positions(exploration_depth);
            export_positions(&positions, &output_path);
        }
        Mode::LoadPositions { path } => {
            let positions = import_positions(&path);
            let responses = get_responses_at_depth(positions.as_ref(), search_depth);
            export_responses(&responses, &output_path);
        }
        Mode::LoadResponses { path } => {
            let responses = import_responses(&path);
            let responses = backtrack_responses(&responses, search_depth, exploration_depth);
            export_responses(&responses, &output_path);
        }
    }
}
