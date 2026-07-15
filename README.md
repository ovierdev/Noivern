# 🎧 Noivern

> **A modular real-time audio detection engine written in Rust.**

Designed for Linux, Raspberry Pi and embedded systems.

Noivern is a lightweight command-line application for real-time audio detection and analysis. It provides a modular architecture that starts with hardware detection and basic DSP features, allowing the project to evolve naturally toward AI-powered audio recognition.

---

# Features

Current version: **v0.1.0**

* Interactive microphone setup
* Automatic input device detection
* Manual device selection
* Automatic `config.toml` generation
* Real-time audio monitoring
* RMS (Root Mean Square)
* ZCR (Zero Crossing Rate)
* Basic sound classification
* Hardware diagnostics
* Audio recording to WAV

Current sound labels:

* `SILENCE`
* `SOUND`
* `NOISY`

---

# Architecture

```text
                +----------------+
                |   Microphone   |
                +----------------+
                         │
                         ▼
                +----------------+
                |  hardware.rs   |
                +----------------+
                         │
                         ▼
                +----------------+
                |  features.rs   |
                |  RMS + ZCR     |
                +----------------+
                         │
                         ▼
                +----------------+
                | classifier.rs  |
                +----------------+
                         │
                         ▼
                +----------------+
                | detector.rs    |
                +----------------+
                         │
                         ▼
                    Terminal Output
```

Project structure:

```text
src/

audio_utils.rs
classifier.rs
config.rs
detector.rs
doctor.rs
features.rs
hardware.rs
record.rs
setup.rs
test.rs

main.rs
```

---
# Getting Started

At this stage (v0.1.0), Noivern is intended for early testing and community feedback.

Precompiled binaries will be available in future releases. For now, please build the project locally.

---

# Requirements

* Linux (Fedora, Ubuntu, Debian, Raspberry Pi OS or compatible)
* Rust (stable)
* Cargo
* ALSA development libraries

Ubuntu / Debian / Raspberry Pi OS

```bash
sudo apt update

sudo apt install -y \
    build-essential \
    pkg-config \
    libasound2-dev
```

Fedora

```bash
sudo dnf install \
    gcc \
    pkgconf-pkg-config \
    alsa-lib-devel
```

Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Reload your shell

```bash
source "$HOME/.cargo/env"
```

Verify the installation

```bash
rustc --version
cargo --version
```

---

# Clone the project

```bash
git clone https://github.com/ovierdev/Noivern.git

cd Noivern
```

---

# Build

Development build

```bash
cargo build
```

Optimized release build

```bash
cargo build --release
```

Executable location

```text
target/release/audio-detector
```

---

# Quick Test

List available microphones

```bash
./target/release/audio-detector devices
```

Configure the microphone

```bash
./target/release/audio-detector setup
```

Run diagnostics

```bash
./target/release/audio-detector doctor
```

Test the microphone

```bash
./target/release/audio-detector test
```

Record a sample

```bash
./target/release/audio-detector record
```

Run the detector

```bash
./target/release/audio-detector run
```

---

# Raspberry Pi

Noivern can also be compiled directly on a Raspberry Pi running a 64-bit Linux distribution.

The build process is exactly the same:

```bash
git clone https://github.com/ovierdev/Noivern.git

cd Noivern

cargo build --release
```

The executable will be generated in:

```text
target/release/audio-detector
```

---

# Feedback

This is an early public release.

If you encounter any issues, please include the following information when opening a GitHub Issue:

* Operating System
* Linux distribution
* CPU architecture (`uname -m`)
* Rust version (`rustc --version`)
* Output of:

```bash
./target/release/audio-detector doctor
```

and

```bash
./target/release/audio-detector devices
```

This information will help reproduce and fix issues much faster.

---

# Usage

## Initial setup

```bash
audio-detector setup
```

Automatically choose a recommended microphone:

```bash
audio-detector setup --auto
```

Select a device by name:

```bash
audio-detector setup --device "USB"
```

---

## List available microphones

```bash
audio-detector devices
```

---

## Test the microphone

```bash
audio-detector test
```

Example:

```text
🎧 Audio Test

Device:
USB Audio Device

Recording...

Results

RMS    : 0.024531
Peak   : 0.831224
Status : Ok
```

---

## System diagnostics

```bash
audio-detector doctor
```

Example:

```text
Application........audio-detector
Version............0.1.0

Configuration......OK
Device.............FOUND

Microphone.........OK

Status.............READY
```

---

## Record audio

Record 5 seconds:

```bash
audio-detector record
```

Record 10 seconds:

```bash
audio-detector record --seconds 10
```

Custom output:

```bash
audio-detector record --seconds 3 --output my-test.wav
```

---

## Real-time detection

```bash
audio-detector run
```

Example output:

```text
[SILENCE] rms=0.0003 zcr=0.02
[SOUND]   rms=0.0321 zcr=0.08
[NOISY]   rms=0.0194 zcr=0.22
```

---

# Mini Manual

## Typical workflow

### 1. Detect available devices

```bash
audio-detector devices
```

### 2. Configure the microphone

```bash
audio-detector setup
```

### 3. Verify the installation

```bash
audio-detector doctor
```

### 4. Test the microphone

```bash
audio-detector test
```

### 5. Record a sample

```bash
audio-detector record
```

### 6. Start real-time detection

```bash
audio-detector run
```

---

# Roadmap

## v0.1.0

* Interactive setup
* Audio recording
* Device diagnostics
* RMS
* ZCR
* Basic classification

## v0.2.0

* FFT
* Spectrogram
* MFCC
* Event logging
* Configuration improvements

## v0.3.0

* Local audio database
* Feature comparison
* Classical Machine Learning
* KNN classifier

## v0.4.0

* ONNX Runtime
* TensorFlow Lite
* Embedded inference

## v1.0.0

Complete modular audio recognition platform.

---

# Contributing

Contributions, ideas and bug reports are always welcome.

Testing on different hardware is especially valuable.

Useful information when reporting an issue:

* Operating System
* CPU Architecture
* Microphone model
* Output from `audio-detector doctor`
* Output from `audio-detector devices`

---

# License

MIT License
