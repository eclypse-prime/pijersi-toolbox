use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct Cli {
    #[command(subcommand)]
    pub mode: Mode,
}

#[derive(Subcommand, Debug)]
pub enum Mode {
    New(GeneratePositionsArgs),
    Load(LoadArgs),
    Inspect(InspectArgs),
    Merge(MergeArgs),
}

#[derive(Args, Debug)]
pub struct LoadArgs {
    pub path: String,
    #[command(subcommand)]
    pub mode: LoadMode,
}

#[derive(Args, Debug)]
pub struct InspectArgs {
    pub path: String,
    #[command(subcommand)]
    pub mode: InspectMode,
}

#[derive(Args, Debug)]
pub struct MergeArgs {
    pub path: String,
    #[command(subcommand)]
    pub mode: MergeMode,
}

#[derive(Subcommand, Debug)]
pub enum LoadMode {
    Positions(GenerateResponsesArgs),
    Responses(GenerateBacktrackingArgs),
}

#[derive(Subcommand, Debug)]
pub enum InspectMode {
    Positions,
    Responses,
}

#[derive(Subcommand, Debug)]
pub enum MergeMode {
    Positions(MergePositionsArgs),
    Responses(MergeResponsesArgs),
}

#[derive(Args, Debug)]
pub struct GeneratePositionsArgs {
    #[arg(short = 'e', long)]
    pub exploration_depth: u64,
    #[arg(short = 'o', long)]
    pub output: String,
    #[arg(long)]
    pub split: Option<u64>,
}

#[derive(Args, Debug)]
pub struct GenerateResponsesArgs {
    #[arg(short = 's', long)]
    pub search_depth: u64,
    #[arg(short = 'o', long)]
    pub output: String,
}

#[derive(Args, Debug)]
pub struct GenerateBacktrackingArgs {
    #[arg(short = 'e', long)]
    pub exploration_depth: u64,
    #[arg(short = 's', long)]
    pub search_depth: u64,
    #[arg(short = 'o', long)]
    pub output: String,
}

#[derive(Args, Debug)]
pub struct MergePositionsArgs {
    #[arg(short = 'o', long)]
    pub output: String,
    #[arg(short = 'n', long)]
    pub number: usize,
}

#[derive(Args, Debug)]
pub struct MergeResponsesArgs {
    #[arg(short = 'o', long)]
    pub output: String,
    #[arg(short = 'n', long)]
    pub number: usize,
}
