use crate::config::build_config;
use crate::hardware::{get_input_devices, select_device, test_microphone};
use anyhow::{Context, Result};
use clap::Args;
use cpal::traits::DeviceTrait;
use std::fs;
use std::time::Duration;

#[derive(Args)]
pub struct SetupArgs {
    #[arg(long)]
    pub auto: bool,

    #[arg(long)]
    pub device: Option<String>,
}

pub fn setup(args: SetupArgs) -> Result<()> {
    println!("Audio Detector Setup");
    println!("Buscando dispositivos de entrada...");

    let devices = get_input_devices()?;

    if devices.is_empty() {
        anyhow::bail!("No se encontraron microfonos conectados.");
    }

    let device = select_device(&devices, args.auto, args.device.as_deref())?;

    let device_name = device
        .name()
        .unwrap_or_else(|_| "Dispositivo desconocido".into());

    let default_config = device
        .default_input_config()
        .context("No se pudo obtener la configuracion del microfono")?;

    let sample_rate = default_config.sample_rate().0;
    let channels = default_config.channels();
    let sample_format = format!("{:?}", default_config.sample_format()).to_lowercase();

    println!("\n Dispositivo seleccionado:");
    println!("Nombre: {device_name}");
    println!("Sample rate: {sample_rate} Hz");
    println!("Canales: {channels}");
    println!("Formato: {sample_format}");

    println!("\n Probando microfono durante 3 segundos...");

    let test = test_microphone(&device, &default_config, Duration::from_secs(3))?;

    println!("RMS: {:.6}", test.rms);
    println!("Peak: {:.6}", test.peak);
    println!("Estado: {:?}", test.status);

    let config = build_config(device_name, sample_rate, channels, sample_format);

    let toml_text =
        toml::to_string_pretty(&config).context("No se pudo serializar la configuracion")?;

    fs::write("config.toml", toml_text).context("No se pudo escribir config.toml")?;

    println!("\nArchivo generado: config.toml");

    Ok(())
}
