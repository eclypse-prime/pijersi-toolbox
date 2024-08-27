use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
pub struct Cli {
    #[arg(short = 'o', long)]
    pub output: String,
    #[arg(short = 'e', long)]
    pub exploration_depth: u64,
    #[arg(short = 's', long)]
    pub search_depth: u64,
    #[command(subcommand)]
    pub mode: Mode,
}

#[derive(Subcommand, Debug)]
pub enum Mode {
    New,
    LoadPositions { path: String },
    LoadResponses { path: String },
}

// #[derive(ValueEnum, Clone, Debug)]
// pub enum Format {
//     Positions,
//     Responses,
// }

// #[derive(ValueEnum, Clone, Debug)]
// pub enum Action {
//     Backtrack,
//     Responses,
// }
