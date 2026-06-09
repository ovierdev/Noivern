use anyhow::{Context, Result};
use clap::{Args, Parser, Subcommand};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, StreamConfig};
use serde::Serialize;
use std::fs;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

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
}

#[derive(Args)]
struct SetupArgs {
    /// Selecciona automáticamente el micrófono recomendado
    #[arg(long)]
    auto: bool,

    /// Selecciona dispositivo por nombre parcial
    #[arg(long)]
    device: Option<String>,
}

#[derive(Serialize)]
struct AppConfig {
    app: AppSection,
    input: InputSection,
    buffer: BufferSection,
    processing: ProcessingSection,
    features: FeaturesSection,
    classifier: ClassifierSection,
    storage: StorageSection,
    ai: AiSection,
    output: OutputSection,
}

#[derive(Serialize)]
struct AppSection {
    name: String,
    version: String,
    log_level: String,
}

#[derive(Serialize)]
struct InputSection {
    device_name: String,
    sample_rate: u32,
    channels: u16,
    sample_format: String,
    backend: String,
}

#[derive(Serialize)]
struct BufferSection {
    buffer_size: u32,
    window_size: u32,
    hop_size: u32,
}

#[derive(Serialize)]
struct ProcessingSection {
    normalize: bool,
    highpass_hz: u32,
    lowpass_hz: u32,
}

#[derive(Serialize)]
struct FeaturesSection {
    enable_energy: bool,
    enable_zcr: bool,
    enable_fft: bool,
    enable_mfcc: bool,
    mfcc_coeffs: u8,
}

#[derive(Serialize)]
struct ClassifierSection {
    #[serde(rename = "type")]
    classifier_type: String,
    threshold: f32,
}

#[derive(Serialize)]
struct StorageSection {
    dataset_path: String,
    cache_features: bool,
}

#[derive(Serialize)]
struct AiSection {
    enabled: bool,
    model_path: String,
    input_type: String,
}

#[derive(Serialize)]
struct OutputSection {
    mode: String,
    events_file: String,
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

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Setup(args) => setup(args)?,
    }

    Ok(())
}

fn setup(args: SetupArgs) -> Result<()> {
    println!("🎧 Audio Detector Setup");
    println!("Buscando dispositivos de entrada...\n");

    let host = cpal::default_host();

    let devices: Vec<Device> = host
        .input_devices()
        .context("No se pudieron leer dispositivos de entrada")?
        .collect();

    if devices.is_empty() {
        anyhow::bail!("No se encontraron micrófonos conectados.");
    }

    let device = select_device(&devices, &args)?;
    let device_name = device
        .name()
        .unwrap_or_else(|_| "Dispositivo desconocido".into());

    let default_config = device
        .default_input_config()
        .context("No se pudo obtener configuración por defecto del micrófono")?;

    let sample_rate = default_config.sample_rate().0;
    let channels = default_config.channels();
    let sample_format = format!("{:?}", default_config.sample_format()).to_lowercase();

    println!("\n✅ Dispositivo seleccionado:");
    println!("Nombre: {}", device_name);
    println!("Sample rate: {} Hz", sample_rate);
    println!("Canales: {}", channels);
    println!("Formato: {}", sample_format);

    println!("\n🧪 Probando micrófono por 3 segundos...");
    let test = test_microphone(&device, &default_config.into())?;

    println!("RMS: {:.6}", test.rms);
    println!("Peak: {:.6}", test.peak);
    println!("Estado: {:?}", test.status);

    let config = build_config(device_name, sample_rate, channels, sample_format);
    let toml_text = toml::to_string_pretty(&config)?;

    fs::write("config.toml", toml_text)?;

    println!("\n💾 Archivo generado: config.toml");

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

        anyhow::bail!("No se encontró dispositivo que contenga: {}", name_filter);
    }

    if args.auto {
        let recommended = choose_recommended_device(devices)?;
        return Ok(recommended);
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
            println!("Auto: usando micrófono USB recomendado.");
            return Ok(device.clone());
        }
    }

    println!("Auto: usando primer dispositivo disponible.");
    Ok(devices[0].clone())
}

fn test_microphone(device: &Device, config: &StreamConfig) -> Result<MicTestResult> {
    let samples = Arc::new(Mutex::new(Vec::<f32>::new()));
    let samples_callback = Arc::clone(&samples);

    let err_fn = |err| eprintln!("Error en stream de audio: {}", err);

    let stream = device.build_input_stream(
        config,
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            let mut buffer = samples_callback.lock().unwrap();
            buffer.extend_from_slice(data);
        },
        err_fn,
        None,
    )?;

    stream.play()?;
    thread::sleep(Duration::from_secs(3));
    drop(stream);

    let buffer = samples.lock().unwrap();

    if buffer.is_empty() {
        anyhow::bail!("No se capturaron muestras del micrófono.");
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

fn calculate_rms(samples: &[f32]) -> f32 {
    let sum_squares: f32 = samples.iter().map(|sample| sample * sample).sum();
    (sum_squares / samples.len() as f32).sqrt()
}

fn calculate_peak(samples: &[f32]) -> f32 {
    samples
        .iter()
        .map(|sample| sample.abs())
        .fold(0.0_f32, f32::max)
}

fn build_config(
    device_name: String,
    sample_rate: u32,
    channels: u16,
    sample_format: String,
) -> AppConfig {
    AppConfig {
        app: AppSection {
            name: "audio-detector".into(),
            version: "0.1.0".into(),
            log_level: "info".into(),
        },
        input: InputSection {
            device_name,
            sample_rate,
            channels,
            sample_format,
            backend: "auto".into(),
        },
        buffer: BufferSection {
            buffer_size: 1024,
            window_size: 2048,
            hop_size: 512,
        },
        processing: ProcessingSection {
            normalize: true,
            highpass_hz: 0,
            lowpass_hz: 0,
        },
        features: FeaturesSection {
            enable_energy: true,
            enable_zcr: true,
            enable_fft: false,
            enable_mfcc: false,
            mfcc_coeffs: 13,
        },
        classifier: ClassifierSection {
            classifier_type: "basic".into(),
            threshold: 0.7,
        },
        storage: StorageSection {
            dataset_path: "./data".into(),
            cache_features: true,
        },
        ai: AiSection {
            enabled: false,
            model_path: "./model.onnx".into(),
            input_type: "spectrogram".into(),
        },
        output: OutputSection {
            mode: "stdout".into(),
            events_file: "events.log".into(),
        },
    }
}
