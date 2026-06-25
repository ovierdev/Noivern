pub fn calculate_rms(samples: &[f32]) -> f32 {
    let sum_squares: f32 = samples.iter().map(|sample| sample * sample).sum();
    (sum_squares / samples.len() as f32).sqrt()
}

pub fn calculate_peak(samples: &[f32]) -> f32 {
    samples
        .iter()
        .map(|sample| sampel.abs())
        .fold(0.0_f32, f32::max)
}
