use crate::config::AppConfig;
use crate::hardware::{MicStatus, find_device_by_name, get_input_devices, test_microphone};
use anyhow::{Context, Result};
use cpal::traits::DeviceTrait;
use std::fs;
use std::time::Duration;

pub fn run_doctor() -> Result<()> {
    println!("Noivern Doctor");
    println!("==========================\n");

    print_system_information();

    println!("\nConfiguration");
    println!("---------------------");

    let config = match load_config() {
        Ok(config) => {
            println!("Config file........OK");
            config
        }
        Err(error) => {
            println!("Config file ........ERROR");
            println!("Reason..............{error}");
            println!("\nStatus.............NOT READY");

            return Ok(());
        }
    };

    println!("Configred device..{}", config.input.device_name);
    println!("Sample rate.......{} Hz", config.input.sample_rate);
    println!("Channels..........{}", config.input.channels);
    println!("Sample format.....{}", config.input.sample_format);

    println!("\nHardware");
    println!("----------------------");

    let device = get_input_devices()?;

    println!("Input devices......{}", device.len());

    if device.is_empty() {
        println!("Configured device.NOT FOUND");
        println!("\nStatus..............NOT READY");

        return Ok(());
    }

    let device = match find_device_by_name(&device, &config.input.device_name) {
        Ok(device) => {
            println!("Configured device.FOUND");
            device
        }
        Err(error) => {
            println!("Configured device.NOT FOUND");
            println!("Reason ..............{error}");
            println!("\nStatus..............NOT READY");

            return Ok(());
        }
    };

    let device_name = device
        .name()
        .unwrap_or_else(|_| "Uknown device".to_string());
    println!("Active device...........{device_name}");

    let supported_config = match device.default_input_config() {
        Ok(config) => {
            println!("Input config..........OK");
            config
        }
        Err(error) => {
            println!("Input config...........ERROR");
            println!("Reason...............{error}");
            println!("\nStatus.............NOT READY");

            return Ok(());
        }
    };

    let actual_sample_rate = supported_config.sample_rate().0;
    let actual_channels = supported_config.channels();
    let actual_format = format!("{:?}", supported_config.sample_format()).to_lowercase();

    println!("Actual rate............{actual_sample_rate} Hz");
    println!("Actual channels........{actual_channels}");
    println!("Actual format..........{actual_format}");

    print_configuration_warnings(&config, actual_sample_rate, actual_channels, &actual_format);

    println!("\nMicrophone test");
    println!("------------------------------");
    println!("Recording.........2 seconds");

    let result = match test_microphone(&device, &supported_config, Duration::from_secs(2)) {
        Ok(result) => result,
        Err(error) => {
            println!("Input stream......ERROR");
            println!("Reason............{error}");
            println!("\nStatus............NOT READY");

            return Ok(());
        }
    };

    println!("Input stream......OK");
    println!("RMS...............{:.6}", result.rms);
    println!("Peak..............{:.6}", result.peak);
    println!("Microphone........{:?}", result.status);

    println!("\nFinal result");
    println!("------------------------------");

    match result.status {
        MicStatus::Ok => {
            println!("Status............READY");
        }
        MicStatus::NoSignal => {
            println!("Status............WARNING");
            println!("Advice............Check mute and microphone level");
        }
        MicStatus::Saturated => {
            println!("Status............WARNING");
            println!("Advice............Reduce microphone gain");
        }
    }

    Ok(())
}

fn load_config() -> Result<AppConfig> {
    let config_text = fs::read_to_string("config.toml")
        .context("No se pudo leer config.toml. Ejecuta primero: audio-detector setup")?;

    let config: AppConfig =
        toml::from_str(&config_text).context("config.toml contiene datos inválidos")?;

    Ok(config)
}

fn print_system_information() {
    println!("System");
    println!("------------------------------");
    println!("Application........{}", env!("CARGO_PKG_NAME"));
    println!("Version............{}", env!("CARGO_PKG_VERSION"));
    println!("Operating system...{}", std::env::consts::OS);
    println!("Architecture.......{}", std::env::consts::ARCH);
}

fn print_configuration_warnings(
    config: &AppConfig,
    actual_sample_rate: u32,
    actual_channels: u16,
    actual_format: &str,
) {
    println!("\nConfiguration comparison");
    println!("------------------------------");

    let mut warnings = 0;

    if config.input.sample_rate == actual_sample_rate {
        println!("Sample rate.......MATCH");
    } else {
        warnings += 1;

        println!(
            "Sample rate.......MISMATCH (config={}, actual={})",
            config.input.sample_rate, actual_sample_rate,
        );
    }

    if config.input.channels == actual_channels {
        println!("Channels..........MATCH");
    } else {
        warnings += 1;

        println!(
            "Channels..........MISMATCH (config={}, actual={})",
            config.input.channels, actual_channels,
        );
    }

    if config
        .input
        .sample_format
        .eq_ignore_ascii_case(actual_format)
    {
        println!("Sample format.....MATCH");
    } else {
        warnings += 1;

        println!(
            "Sample format.....MISMATCH (config={}, actual={})",
            config.input.sample_format, actual_format,
        );
    }

    println!("Warnings..........{warnings}");
}
