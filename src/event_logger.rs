use crate::event::DetectorEvent;
use anyhow::{Context, Result};
use chrono::{Local, SecondsFormat};
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::Path;

pub struct EventLogger {
    writer: BufWriter<File>,
}

impl EventLogger {
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();

        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .with_context(|| format!("No se pudo abrir el archivo de log: {}", path.display()))?;

        Ok(Self {
            writer: BufWriter::new(file),
        })
    }

    pub fn log(&mut self, event: &DetectorEvent) -> Result<()> {
        let timestamp = Local::now().to_rfc3339_opts(SecondsFormat::Millis, true);

        let message = format_event(event);

        writeln!(self.writer, "{timestamp} {message}")
            .context("No se pudo escribir el evento en el log")?;

        self.writer
            .flush()
            .context("No se pudo guardar el evento en el log")?;

        Ok(())
    }
}

fn format_event(event: &DetectorEvent) -> String {
    match event {
        DetectorEvent::ServiceStarted { device_name } => {
            format!("SERVICE=STARTED device=\"{}\"", sanitize_value(device_name))
        }

        DetectorEvent::ServiceStopped => "SERVICE=STOPPED".to_string(),

        DetectorEvent::StateChanged {
            previous,
            current,
            rms,
            zcr,
        } => {
            format!(
                "EVENT=STATE_CHANGED previous={previous} current={current} \
                 rms={rms:.6} zcr={zcr:.6}"
            )
        }

        DetectorEvent::PeakDetected { rms, peak, zcr } => {
            format!("EVENT=PEAK rms={rms:.6} peak={peak:.6} zcr={zcr:.6}")
        }

        DetectorEvent::AudioError { message } => {
            format!("EVENT=AUDIO_ERROR message=\"{}\"", sanitize_value(message))
        }
    }
}

fn sanitize_value(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', " ")
        .replace('\r', " ")
}
