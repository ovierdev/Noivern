use crate::audio_utils::{calculate_peak, calculate_rms};
use anyhow::{Context, Result};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, Sample, SampleFormat, StreamConfig};
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
pub struct MicTestResult {
    pub peak: f32,
    pub rms: f32,
    pub status: MicStatus,
}

#[derive(Debug)]
pub enum MicStatus {
    Ok,
    NoSignal,
    Saturated,
}

pub fn get_input_devices() -> Result<Vec<Device>> {
    let host = cpal::default_host();

    let devices: Vec<Device> = host
        .input_devices()
        .context("No se pudieron leer los dispositivos de entrada")?
        .collect();
    Ok(devices)
}

pub fn list_devices() -> Result<()> {
    println!("Input Devices\n");

    let devices = get_input_devices()?;

    if devices.is_empty() {
        println!("No se encontraron dispositivos de entrada.");
        return Ok(());
    }

    for (index, device) in devices.iter().enumerate() {
        let name = device
            .name()
            .unwrap_or_else(|_| "Dispositivo desconocido".into());
        println!("[{index}] {name}");

        match device.default_input_config() {
            Ok(config) => {
                println!("  sample rate: {} Hz", config.sample_rate().0);
                println!(" channels: {}", config.channels());
                println!("  format: {:?}", config.sample_format());
            }
            Err(error) => {
                println!("  no se pudo leer la configuracion: {error}");
            }
        }
        println!();
    }
    Ok(())
}

pub fn select_device(
    devices: &[Device],
    auto: bool,
    device_filter: Option<&str>,
) -> Result<Device> {
    if let Some(filter) = device_filter {
        return find_device_by_name(devices, filter);
    }
    if auto {
        return choose_recommended_device(devices);
    }

    select_device_interactive(devices)
}

fn find_device_by_name(devices: &[Device], filter: &str) -> Result<Device> {
    let filter = filter.to_lowercase();

    for device in devices {
        let name = device.name().unwrap_or_default().to_lowercase();

        if name.to_lowercase().contains(&filter) {
            println!("Dispositivo encontrado: {name}");
            return Ok(device.clone());
        }
    }
    anyhow::bail!("No se encontro un dispositivo que contenga: {filter}");
}

fn select_device_interactive(devices: &[Device]) -> Result<Device> {
    for (index, device) in devices.iter().enumerate() {
        let name = device
            .name()
            .unwrap_or_else(|_| "Dispositivo desconocido".into());

        println!("[{index}] {name}");
    }
    println!("\nSeleccione Dispositivo: ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let selected_index: usize = input
        .trim()
        .parse()
        .context("Debe ingresar un numero valido")?;

    let device = devices
        .get(selected_index)
        .context("Indice de dispositivo invalido")?;

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
    let device = devices
        .first()
        .context("No hay dispositivos de entrada disponibles")?;
    println!("Auto: usando el primer dispositivo disponible.");

    Ok(device.clone())
}

pub fn test_microphone(
    device: &Device,
    supported_config: &cpal::SupportedStreamConfig,
    duration: Duration,
) -> Result<MicTestResult> {
    let sample_format = supported_config.sample_format();
    let stream_config: StreamConfig = supported_config.clone().into();

    match sample_format {
        SampleFormat::F32 => test_microphone_with_format::<f32>(device, &stream_config, duration),
        SampleFormat::I16 => test_microphone_with_format::<i16>(device, &stream_config, duration),
        SampleFormat::U16 => test_microphone_with_format::<u16>(device, &stream_config, duration),
        _ => anyhow::bail!("Formato de muestra no soportado: {sample_format:?}"),
    }
}

fn test_microphone_with_format<T>(
    device: &Device,
    config: &StreamConfig,
    duration: Duration,
) -> Result<MicTestResult>
where
    T: cpal::Sample + cpal::SizedSample,
    f32: cpal::FromSample<T>,
{
    let samples = Arc::new(Mutex::new(Vec::<f32>::new()));
    let samples_callback = Arc::clone(&samples);

    let err_fn = |error| {
        eprintln!("Error en el stream de audio: {error}");
    };

    let stream = device.build_input_stream(
        config,
        move |data: &[T], _: &cpal::InputCallbackInfo| {
            let Ok(mut buffer) = samples_callback.lock() else {
                eprintln!("No se pudo bloquear el buffer de audio");
                return;
            };
            buffer.extend(data.iter().map(|sample| f32::from_sample(*sample)));
        },
        err_fn,
        None,
    )?;

    stream.play()?;
    thread::sleep(duration);
    drop(stream);

    let buffer = samples
        .lock()
        .map_err(|_| anyhow::anyhow!("No se pudo acceder al buffer de audio"))?;

    if buffer.is_empty() {
        anyhow::bail!("No se capturaron muestras del microfono.");
    }

    let rms = calculate_rms(&buffer);
    let peak = calculate_peak(&buffer);

    let status = determine_microphone_status(rms, peak);

    Ok(MicTestResult { rms, peak, status })
}

fn determine_microphone_status(rms: f32, peak: f32) -> MicStatus {
    if peak > 0.98 {
        MicStatus::Saturated
    } else if rms < 0.0005 {
        MicStatus::NoSignal
    } else {
        MicStatus::Ok
    }
}
