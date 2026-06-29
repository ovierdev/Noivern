use crate::features::AudioFeatures;

#[derive(Debug, Clone, Copy)]
pub enum SoundClass {
    Silence,
    Sound,
    Noisy,
}

pub fn classify_basic(features: AudioFeatures, rms_threshold: f32) -> SoundClass {
    if features.rms < rms_threshold {
        SoundClass::Silence
    } else if features.zcr > 0.02 {
        SoundClass::Noisy
    } else {
        SoundClass::Sound
    }
}
