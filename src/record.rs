use crate::config::AppConfig;
use crate::hardware::{capture_audio, find_device_by_name, get_input_devices};
use anyhow::{Context, Result};
use clap::Args;
use cpal::traits::DeviceTrait;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

#[derive(Args)]
pub struct RecordArgs {
    // Duration de la grabacion en segundos
    #[arg(short, long, default_value_t = 5)]
    pub seconds: u64,

    // Nombre o ruta del archivo WAV de salida
    #[arg(short, long, default_value = "recording.wav")]
    pub output: PathBuf,
}

pub fn run_record(args: RecordArgs) -> Result<()> {
    if args.seconds == 0 {
        anyhow::bail!("La duration debe ser mayor que cero.");
    }

    println!("Noivern Recorder");
    println!("====================");

    let app_config = load_config()?;
    let device = get_input_devices()?;

    if device.is_empty() {
        anyhow::bail!("No se encontraron dispositivos de entrada.");
    }

    let device = find_device_by_name(&device, &app_config.input.device_name)
        .context("No se encontro el dispositivo configurado")?;

    let device_name = device
        .name()
        .unwrap_or_else(|_| "Dispositivo desconocido".into());

    let supported_config = device
        .default_input_config()
        .context("No se pudo obtener la configuracion del microfono")?;

    let sample_rate = supported_config.sample_rate().0;
    let channels = supported_config.channels();

    println!("Device....................{device_name}");
    println!("Sample rate...............{sample_rate} Hz");
    println!("Channels..................{channels}");
    println!("Duration..................{} seconds", args.seconds);
    println!("Output....................{}", args.output.display());
    println!("\nRecording....");

    let samples = capture_audio(
        &device,
        &supported_config,
        Duration::from_secs(args.seconds),
    )?;

    write_wav(&args.output, &samples, sample_rate, channels)?;

    let frame_count = samples.len() / usize::from(channels);

    println!("\nRecording.........OK");
    println!("Samples...........{}", samples.len());
    println!("Frames............{frame_count}");
    println!("Saved.............{}", args.output.display());

    Ok(())
}

fn load_config() -> Result<AppConfig> {
    let config_text = fs::read_to_string("config.toml")
        .context("No se pudo leer config.toml. Ejecuta primero: audio-detector setup")?;
    toml::from_str(&config_text).context("config.toml contiene datos invalidos")
}

fn write_wav(output: &PathBuf, samples: &[f32], sample_rate: u32, channels: u16) -> Result<()> {
    let wav_spec = hound::WavSpec {
        channels,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create(output, wav_spec)
        .with_context(|| format!("No se pudo crear el archivo {}", output.display()))?;

    for sample in samples {
        let normalized = sample.clamp(-1.0, 1.0);

        let sample_i16 = (normalized * i16::MAX as f32) as i16;

        writer.write_sample(sample_i16)?;
    }

    writer
        .finalize()
        .context("No se pudo finalizar el archivo WAV")?;

    Ok(())
}
