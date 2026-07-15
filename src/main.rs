mod audio_utils;
mod classifier;
mod config;
mod detector;
mod doctor;
mod features;
mod hardware;
mod record;
mod setup;
mod test;

use anyhow::Result;
use clap::{Parser, Subcommand};
use record::RecordArgs;
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
    Devices,
    Test,
    Doctor,
    Record(RecordArgs),
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Setup(args) => setup::setup(args)?,
        Commands::Run => detector::run_detector()?,
        Commands::Devices => hardware::list_devices()?,
        Commands::Test => test::run_test()?,
        Commands::Doctor => doctor::run_doctor()?,
        Commands::Record(args) => record::run_record(args)?,
    }

    Ok(())
}
