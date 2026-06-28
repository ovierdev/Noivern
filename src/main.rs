mod audio_utils;
mod config;
mod detector;
mod setup;

use anyhow::Result;
use clap::{Parser, Subcommand};
use setup::SetupArgs;

#[derive(Parser)]
#[command(name = "audio-detector")]
#[command(version = "0.1.0")]
#[command(about = "Detector de audio en Rust")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Setup(SetupArgs),
    Run,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Setup(args) => setup::setup(args)?,
        Commands::Run => detector::run_detector()?,
    }

    Ok(())
}
