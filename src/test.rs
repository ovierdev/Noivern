use anyhow::{Context, Result};
use cpal::traits::DeviceTrait;
use std::time::Duration;

use crate::hardware::{get_input_devices, select_device, test_microphone};

pub fn run_test() -> Result<()> {
    println!("Audio Test\n");

    let devices = get_input_devices()?;

    if devices.is_empty() {
        anyhow::bail!("No input devices found.");
    }

    let device = select_device(&devices, false, None)?;

    let config = device
        .default_input_config()
        .context("Unable to read microphone configuration")?;

    println!("\nDevice: {}\n", device.name()?);
    println!("Recording for 3 seconds...\n");

    let result = test_microphone(&device, &config, Duration::from_secs(3))?;

    println!("Results");
    println!("------------------------");
    println!("RMS    : {:.6}", result.rms);
    println!("Peak   : {:.6}", result.peak);
    println!("Status : {:?}", result.status);

    Ok(())
}
