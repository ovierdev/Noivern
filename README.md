# Noivern

> A modular real-time audio detection engine written in Rust.

Noivern is a lightweight audio analysis tool designed to run on desktop computers and single-board computers such as the Raspberry Pi.

The long-term goal is to evolve from a simple sound detector into a modular AI-powered audio recognition platform.

---

## Features

Current version (**v0.1.0**)

* Interactive setup wizard
* Automatic microphone detection
* Manual device selection
* Generates `config.toml`
* Real-time audio capture
* RMS feature extraction
* Zero Crossing Rate (ZCR)
* Basic sound classification

Current labels:

* SILENCE
* SOUND
* NOISY

---

## Project Goals

This project is being developed incrementally.

Roadmap:

* Basic audio detection
* Digital Signal Processing (DSP)
* Feature extraction
* Local sound database
* Classical Machine Learning
* AI inference (ONNX / TensorFlow Lite)
* Raspberry Pi deployment

---

## Build

```bash
cargo build --release
```

Executable:

```text
target/release/audio-detector
```

---

## Setup

Run the setup wizard:

```bash
audio-detector setup
```

Automatic configuration:

```bash
audio-detector setup --auto
```

Select device manually:

```bash
audio-detector setup --device "USB"
```

---

## Run

```bash
audio-detector run
```

Example output:

```text
[SILENCE] rms=0.0003 zcr=0.02
[SOUND]   rms=0.0312 zcr=0.08
[NOISY]   rms=0.0201 zcr=0.22
```

---

## Architecture

```text
Setup
   │
   ▼
config.toml
   │
   ▼
Audio Input
   │
   ▼
Feature Extraction
(RMS + ZCR)
   │
   ▼
Classifier
   │
   ▼
Output
```

---

## Current Project Structure

```text
src/

audio_utils.rs
classifier.rs
config.rs
detector.rs
features.rs
setup.rs
main.rs
```

---

## Planned Features

* Device diagnostics (`doctor`)
* Device listing (`devices`)
* Audio recorder (`record`)
* FFT
* Spectrogram
* MFCC
* Sound database
* KNN classifier
* ONNX Runtime integration
* TensorFlow Lite support

---

## Contributing

Suggestions, bug reports and pull requests are always welcome.

Testing on different operating systems and microphones is especially appreciated.

---

## License

MIT License
