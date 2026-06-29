use crate::classifier::{SoundClass, classify_basic};
use crate::config::AppConfig;
use crate::features::extract_features;
use anyhow::{Context, Result};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, Sample, SampleFormat, StreamConfig};
use std::fs;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn run_detector() -> Result<()> {
    println!("Audio Detector RUN");
    println!("Cargando config.toml..\n");

    let config_text = fs::read_to_string("config.toml")
        .context("No se pudo leer config.toml. Ejecuta primero: cargo run -- setup")?;
    let app_config: AppConfig =
        toml::from_str(&config_text).context("No se pudo parsear config.toml")?;
    let host = cpal::default_host();
    let device = find_input_device(&host, &app_config.input.device_name)
        .context("No se pudo encontrar el dispositivo configurado")?;

    println!("Microfono: {}", app_config.input.device_name);
    println!("Threshold: {}", app_config.classifier.threshold);
    println!("Presiona Ctrl+C para detener\n");

    let supported_config = device
        .default_input_config()
        .context("No se pudo obtener configuracion por defecto del dispositivo")?;

    let sample_format = supported_config.sample_format();
    let stream_config: StreamConfig = supported_config.into();

    match sample_format {
        SampleFormat::F32 => run_stream::<f32>(
            &device,
            &stream_config,
            app_config.classifier.threshold,
            app_config.buffer.window_size as usize,
        )?,
        SampleFormat::I16 => run_stream::<i16>(
            &device,
            &stream_config,
            app_config.classifier.threshold,
            app_config.buffer.window_size as usize,
        )?,
        SampleFormat::U16 => run_stream::<u16>(
            &device,
            &stream_config,
            app_config.classifier.threshold,
            app_config.buffer.window_size as usize,
        )?,
        _ => anyhow::bail!("Formato de muestra no soportado: {:?}", sample_format),
    }
    Ok(())
}

fn find_input_device(host: &cpal::Host, device_name: &str) -> Result<Device> {
    let devices = host
        .input_devices()
        .context("No se pudieron leer dispositivos de entrada")?;

    for device in devices {
        let name = device.name().unwrap_or_default();

        if name == device_name || name.to_lowercase().contains(&device_name.to_lowercase()) {
            return Ok(device);
        }
    }

    anyhow::bail!("Dispositivo no encontrado: {}", device_name);
}

fn run_stream<T>(
    device: &Device,
    config: &StreamConfig,
    threshold: f32,
    window_size: usize,
) -> Result<()>
where
    T: cpal::Sample + cpal::SizedSample,
    f32: cpal::FromSample<T>,
{
    let frame_samples = Arc::new(Mutex::new(Vec::<f32>::new()));
    let frame_samples_callback = Arc::clone(&frame_samples);

    let err_fn = |err| eprintln!("Error en stream de audio: {}", err);

    let stream = device.build_input_stream(
        config,
        move |data: &[T], _: &cpal::InputCallbackInfo| {
            let mut buffer = frame_samples_callback.lock().unwrap();

            for sample in data {
                let value: f32 = f32::from_sample(*sample);
                buffer.push(value);
            }

            if buffer.len() >= window_size {
                let frame = &buffer[..window_size];

                let features = extract_features(frame);
                let class = classify_basic(features, threshold);

                match class {
                    SoundClass::Silence => {
                        println!(
                            "[Silence] rms = {:.6} zcr = {:.6}",
                            features.rms, features.zcr
                        );
                    }
                    SoundClass::Sound => {
                        println!(
                            "[Sound] rms = {:.6} zcr = {:.6}",
                            features.rms, features.zcr
                        );
                    }
                    SoundClass::Noisy => {
                        println!("[NOISY] rms =  {:.6} zcr={:.6}", features.rms, features.zcr);
                    }
                }

                buffer.clear();
            }
        },
        err_fn,
        None,
    )?;

    stream.play()?;

    loop {
        thread::sleep(Duration::from_secs(1));
    }
}
