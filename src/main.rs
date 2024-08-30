use clap::Parser;

use pijersi_toolbox::{actions::{backtrack_responses, get_positions, get_responses_at_depth, inspect_position, inspect_response}, args::{Cli, InspectMode, LoadMode, Mode}, io::{export_positions, export_responses, import_positions, import_responses}};

fn main() {
    let args = Cli::parse();

    match args.mode {
        Mode::New(position_args) => {
            let exploration_depth = position_args.exploration_depth;
            let output_path = position_args.output;
            let split = position_args.split;
            let positions = get_positions(exploration_depth);
            export_positions(&positions, &output_path, split);
        }
        Mode::Load(load_args) => {
            let file_path = load_args.path;
            match load_args.mode {
                LoadMode::Positions(generate_response_args) => {
                    let search_depth = generate_response_args.search_depth;
                    let output_path = generate_response_args.output;
                    let positions = import_positions(&file_path);
                    let responses = get_responses_at_depth(positions.as_ref(), search_depth);
                    export_responses(&responses, &output_path);
                }
                LoadMode::Responses(generate_backtracking_args) => {
                    let search_depth = generate_backtracking_args.search_depth;
                    let exploration_depth = generate_backtracking_args.exploration_depth;
                    let output_path = generate_backtracking_args.output;
                    let responses = import_responses(&file_path);
                    let responses = backtrack_responses(&responses, search_depth, exploration_depth);
                    export_responses(&responses, &output_path);
                }
            }
        }
        Mode::Inspect(inspect_args) => {
            let file_path = inspect_args.path;
            match inspect_args.mode {
                InspectMode::Positions => {
                        let positions = import_positions(&file_path);
                        for position in positions.iter().take(5) {
                            inspect_position(position);
                        }
                }
                InspectMode::Responses => {
                        let responses = import_responses(&file_path);
                        for response in responses.iter().take(5) {
                            inspect_response(response);
                        }
                }
            }
        }
    }
}
