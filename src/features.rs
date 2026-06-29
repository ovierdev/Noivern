use crate::audio_utils::{calculate_rms, calculate_zcr};

#[derive(Debug, Clone, Copy)]
pub struct AudioFeatures {
    pub rms: f32,
    pub zcr: f32,
}

pub fn extract_features(samples: &[f32]) -> AudioFeatures {
    AudioFeatures {
        rms: calculate_rms(samples),
        zcr: calculate_zcr(samples),
    }
}
