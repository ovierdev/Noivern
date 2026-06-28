use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    pub app: AppSection,
    pub input: InputSection,
    pub buffer: BufferSection,
    pub processing: ProcessingSection,
    pub features: FeaturesSection,
    pub classifier: ClassifierSection,
    pub storage: StorageSection,
    pub ai: AiSection,
    pub output: OutputSection,
}

#[derive(Serialize, Deserialize)]
pub struct AppSection {
    pub name: String,
    pub version: String,
    pub log_level: String,
}

#[derive(Serialize, Deserialize)]
pub struct InputSection {
    pub device_name: String,
    pub sample_rate: u32,
    pub channels: u16,
    pub sample_format: String,
    pub backend: String,
}

#[derive(Serialize, Deserialize)]
pub struct BufferSection {
    pub buffer_size: u32,
    pub window_size: u32,
    pub hop_size: u32,
}

#[derive(Serialize, Deserialize)]
pub struct ProcessingSection {
    pub normalize: bool,
    pub highpass_hz: u32,
    pub lowpass_hz: u32,
}

#[derive(Serialize, Deserialize)]
pub struct FeaturesSection {
    pub enable_energy: bool,
    pub enable_zcr: bool,
    pub enable_fft: bool,
    pub enable_mfcc: bool,
    pub mfcc_coeffs: u8,
}

#[derive(Serialize, Deserialize)]
pub struct ClassifierSection {
    #[serde(rename = "type")]
    pub classifier_type: String,
    pub threshold: f32,
}

#[derive(Serialize, Deserialize)]
pub struct StorageSection {
    pub dataset_path: String,
    pub cache_features: bool,
}

#[derive(Serialize, Deserialize)]
pub struct AiSection {
    pub enabled: bool,
    pub model_path: String,
    pub input_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct OutputSection {
    pub mode: String,
    pub events_file: String,
}

pub fn build_config(
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
            threshold: 0.01,
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
