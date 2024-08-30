use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct Cli {
    #[command(subcommand)]
    pub mode: Mode,
}

#[derive(Subcommand, Debug)]
pub enum Mode {
    New(GeneratePositionArgs),
    Load(LoadArgs),
    Inspect(InspectArgs),
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

#[derive(Subcommand, Debug)]
pub enum LoadMode {
    Positions(GenerateResponseArgs),
    Responses(GenerateBacktrackingArgs),
}

#[derive(Subcommand, Debug)]
pub enum InspectMode {
    Positions,
    Responses,
}

#[derive(Args, Debug)]
pub struct GeneratePositionArgs {
    #[arg(short = 'e', long)]
    pub exploration_depth: u64,
    #[arg(short = 'o', long)]
    pub output: String,
    #[arg(long)]
    pub split: Option<u64>,
}

#[derive(Args, Debug)]
pub struct GenerateResponseArgs {
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