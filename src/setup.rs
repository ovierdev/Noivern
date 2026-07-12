use crate::audio_utils::{calculate_peak, calculate_rms};
use crate::config::build_config;
use anyhow::{Context, Result};
use clap::Args;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, Sample, SampleFormat, StreamConfig};

use std::fs;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Args)]
pub struct SetupArgs {
    #[arg(long)]
    pub auto: bool,

    #[arg(long)]
    pub device: Option<String>,
}

#[derive(Debug)]
struct MicTestResult {
    rms: f32,
    peak: f32,
    status: MicStatus,
}

#[derive(Debug)]
enum MicStatus {
    Ok,
    NoSignal,
    Saturated,
}

pub fn setup(args: SetupArgs) -> Result<()> {
    println!("Audio Detector Setup");
    println!("Buscando dispositivos de entrada...");

    let host = cpal::default_host();

    let devices: Vec<Device> = host
        .input_devices()
        .context("No se pudieron leer dispositivos de entrada")?
        .collect();

    if devices.is_empty() {
        anyhow::bail!("No se encontraron microfonos conectados.");
    }

    let device = select_device(&devices, &args)?;
    let device_name = device
        .name()
        .unwrap_or_else(|_| "Dispositivo desconocido".into());

    let default_config = device
        .default_input_config()
        .context("No se pudo obtener configuracion por defecto del microfono")?;

    let sample_rate = default_config.sample_rate().0;
    let channels = default_config.channels();
    let sample_format = format!("{:?}", default_config.sample_format()).to_lowercase();

    println!("\n Dispositivo seleccionado:");
    println!("Nombnre: {}", device_name);
    println!("Sample rate: {} Hz", sample_rate);
    println!("Canales: {}", channels);
    println!("Formato: {}", sample_format);

    println!("\n🧪 Probando micrófono por 3 segundos...");
    let test = test_microphone(&device, &default_config)?;

    println!("RMS: {:.6}", test.rms);
    println!("Peak: {:.6}", test.peak);
    println!("Estado: {:?}", test.status);

    let config = build_config(device_name, sample_rate, channels, sample_format);
    let toml_text = toml::to_string_pretty(&config)?;

    fs::write("config.toml", toml_text)?;

    println!("\n Archivo generado: config.toml");
    Ok(())
}

fn select_device(devices: &[Device], args: &SetupArgs) -> Result<Device> {
    if let Some(name_filter) = &args.device {
        let name_filter = name_filter.to_lowercase();

        for device in devices {
            let name = device.name().unwrap_or_default().to_lowercase();

            if name.contains(&name_filter) {
                println!("Dispositivo encontrado por nombre: {}", name);
                return Ok(device.clone());
            }
        }
        anyhow::bail!("No se encontro dispositivo que contenga: {}", name_filter);
    }

    if args.auto {
        return choose_recommended_device(devices);
    }

    for (index, device) in devices.iter().enumerate() {
        let name = device
            .name()
            .unwrap_or_else(|_| "Dispositivo desconocido".into());
        println!("[{}] {}", index, name);
    }

    print!("\nSeleccione dispositivo: ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let selected_index: usize = input
        .trim()
        .parse()
        .context("Debe ingresar un número válido")?;

    let device = devices
        .get(selected_index)
        .context("Índice de dispositivo inválido")?;

    Ok(device.clone())
}

fn choose_recommended_device(devices: &[Device]) -> Result<Device> {
    for device in devices {
        let name = device.name().unwrap_or_default().to_lowercase();

        if name.contains("usb") {
            println!("Auto: usando microfono USB recomendado.");
            return Ok(device.clone());
        }
    }

    println!("Auto: usando microfono USB recomendado.");
    Ok(devices[0].clone())
}

fn test_microphone(
    device: &Device,
    supported_config: &cpal::SupportedStreamConfig,
) -> Result<MicTestResult> {
    let sample_format = supported_config.sample_format();
    let config: StreamConfig = supported_config.clone().into();

    match sample_format {
        SampleFormat::F32 => test_microphone_with_format::<f32>(device, &config),
        SampleFormat::I16 => test_microphone_with_format::<i16>(device, &config),
        SampleFormat::U16 => test_microphone_with_format::<u16>(device, &config),
        _ => anyhow::bail!("Formato de muestra no soportado: {:?}", sample_format),
    }
}

fn test_microphone_with_format<T>(device: &Device, config: &StreamConfig) -> Result<MicTestResult>
where
    T: cpal::Sample + cpal::SizedSample,
    f32: cpal::FromSample<T>,
{
    let samples = Arc::new(Mutex::new(Vec::<f32>::new()));
    let samples_callback = Arc::clone(&samples);

    let err_fn = |err| eprintln!("Error en stream de audio: {}", err);

    let stream = device.build_input_stream(
        config,
        move |data: &[T], _: &cpal::InputCallbackInfo| {
            let mut buffer = samples_callback.lock().unwrap();

            for sample in data {
                let value: f32 = f32::from_sample(*sample);
                buffer.push(value);
            }
        },
        err_fn,
        None,
    )?;

    stream.play()?;
    thread::sleep(Duration::from_secs(3));
    drop(stream);

    let buffer = samples.lock().unwrap();

    if buffer.is_empty() {
        anyhow::bail!("No se capturaron muestras del microfono.");
    }

    let rms = calculate_rms(&buffer);
    let peak = calculate_peak(&buffer);

    let status = if peak > 0.98 {
        MicStatus::Saturated
    } else if rms < 0.0005 {
        MicStatus::NoSignal
    } else {
        MicStatus::Ok
    };

    Ok(MicTestResult { rms, peak, status })
}

pub fn list_devices() -> Result<()> {
    println!("Input Devices\n");

    let host = cpal::default_host();

    let devices: Vec<Device> = host
        .input_devices()
        .context("No se pudieron leer dispositivos de entrada")?
        .collect();

    if devices.is_empty() {
        println!("No se encontraron dispositivos de entrada.");
        return Ok(());
    }

    for (index, devices) in devices.iter().enumerate() {
        let name = devices
            .name()
            .unwrap_or_else(|_| "Dispositivo desconocido".into());

        println!("[{}] {}", index, name);

        match devices.default_input_config() {
            Ok(config) => {
                println!("  sample rate: {} Hz", config.sample_rate().0);
                println!("  channels: {}", config.channels());
                println!("  format: {:?}", config.sample_format());
            }
            Err(_) => {
                println!("  no se puede leer configucaion por defecto");
            }
        }
        println!();
    }
    Ok(())
}
