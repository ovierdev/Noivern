use crate::features::AudioFeatures;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SoundClass {
    Silence,
    Sound,
    Noisy,
}

impl fmt::Display for SoundClass {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SoundClass::Silence => write!(formatter, "SILENCE"),
            SoundClass::Sound => write!(formatter, "SOUND"),
            SoundClass::Noisy => write!(formatter, "NOISY"),
        }
    }
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
