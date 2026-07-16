use crate::classifier::SoundClass;

#[derive(Debug, Clone)]
pub enum DetectorEvent {
    ServiceStarted {
        device_name: String,
    },

    ServiceStopped,

    #[allow(dead_code)]
    StateChanged {
        previous: SoundClass,
        current: SoundClass,
        rms: f32,
        zcr: f32,
    },

    PeakDetected {
        rms: f32,
        peak: f32,
        zcr: f32,
    },

    AudioError {
        message: String,
    },
}
