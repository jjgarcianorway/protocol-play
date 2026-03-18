// SPDX-License-Identifier: GPL-3.0-or-later
//! Procedural sound system — core types and audio generation.
//!
//! All sounds are synthesised at startup as `SynthSound` assets (a custom
//! `Decodable` type).  No external audio files are needed.

use bevy::prelude::*;
use bevy::audio::{AddAudioSource, Decodable, PlaybackSettings, Volume};
use std::sync::Arc;

mod palette;
pub use palette::*;

// ── Constants ────────────────────────────────────────────────────────
const SAMPLE_RATE: u32 = 44_100;
const FADE_SECS: f32 = 0.008; // click-free fade in/out

// ── SynthSound — custom Decodable asset ──────────────────────────────

/// Raw f32 PCM samples at 44 100 Hz, mono.
#[derive(Asset, Debug, Clone, bevy::reflect::TypePath)]
pub struct SynthSound {
    pub samples: Arc<[f32]>,
}

/// Iterator that yields samples from a `SynthSound`.
pub struct SynthDecoder {
    samples: Arc<[f32]>,
    pos: usize,
}

impl Iterator for SynthDecoder {
    type Item = f32;
    fn next(&mut self) -> Option<f32> {
        if self.pos < self.samples.len() {
            let s = self.samples[self.pos];
            self.pos += 1;
            Some(s)
        } else {
            None
        }
    }
}

impl bevy::audio::Source for SynthDecoder {
    fn current_frame_len(&self) -> Option<usize> { Some(self.samples.len() - self.pos) }
    fn channels(&self) -> u16 { 1 }
    fn sample_rate(&self) -> u32 { SAMPLE_RATE }
    fn total_duration(&self) -> Option<std::time::Duration> {
        Some(std::time::Duration::from_secs_f64(
            self.samples.len() as f64 / SAMPLE_RATE as f64,
        ))
    }
}

impl Decodable for SynthSound {
    type DecoderItem = f32;
    type Decoder = SynthDecoder;
    fn decoder(&self) -> Self::Decoder {
        SynthDecoder { samples: self.samples.clone(), pos: 0 }
    }
}

// ── Tone generators ──────────────────────────────────────────────────

/// Pure sine tone with fade envelope.
pub fn tone(freq: f32, dur: f32, vol: f32) -> Vec<f32> {
    let n = (SAMPLE_RATE as f32 * dur) as usize;
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let t = i as f32 / SAMPLE_RATE as f32;
        let env = envelope(t, dur);
        out.push((t * freq * std::f32::consts::TAU).sin() * vol * env);
    }
    out
}

/// Frequency sweep (linear interpolation from `f0` to `f1`).
pub fn sweep(f0: f32, f1: f32, dur: f32, vol: f32) -> Vec<f32> {
    let n = (SAMPLE_RATE as f32 * dur) as usize;
    let mut out = Vec::with_capacity(n);
    let mut phase: f32 = 0.0;
    for i in 0..n {
        let t = i as f32 / SAMPLE_RATE as f32;
        let frac = t / dur;
        let freq = f0 + (f1 - f0) * frac;
        phase += freq * std::f32::consts::TAU / SAMPLE_RATE as f32;
        let env = envelope(t, dur);
        out.push(phase.sin() * vol * env);
    }
    out
}

/// Mix multiple tones into a chord (normalised so peak stays at `vol`).
pub fn chord(freqs: &[f32], dur: f32, vol: f32) -> Vec<f32> {
    let n = (SAMPLE_RATE as f32 * dur) as usize;
    let count = freqs.len() as f32;
    let mut out = vec![0.0f32; n];
    for &freq in freqs {
        for i in 0..n {
            let t = i as f32 / SAMPLE_RATE as f32;
            let env = envelope(t, dur);
            out[i] += (t * freq * std::f32::consts::TAU).sin() * vol * env / count;
        }
    }
    out
}

/// Sequential arpeggio — each note plays for `dur / freqs.len()`.
pub fn arpeggio(freqs: &[f32], dur: f32, vol: f32) -> Vec<f32> {
    let total = (SAMPLE_RATE as f32 * dur) as usize;
    let note_len = total / freqs.len().max(1);
    let mut out = Vec::with_capacity(total);
    for (idx, &freq) in freqs.iter().enumerate() {
        let start = idx * note_len;
        let end = if idx + 1 == freqs.len() { total } else { (idx + 1) * note_len };
        let note_dur = (end - start) as f32 / SAMPLE_RATE as f32;
        for i in 0..(end - start) {
            let t = i as f32 / SAMPLE_RATE as f32;
            let env = envelope(t, note_dur);
            out.push((t * freq * std::f32::consts::TAU).sin() * vol * env);
        }
    }
    out
}

/// Bounce: sine with exponential decay.
pub fn bounce(freq: f32, dur: f32, vol: f32) -> Vec<f32> {
    let n = (SAMPLE_RATE as f32 * dur) as usize;
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let t = i as f32 / SAMPLE_RATE as f32;
        let decay = (-t * 30.0).exp(); // fast decay
        let env_start = if t < FADE_SECS { t / FADE_SECS } else { 1.0 };
        out.push((t * freq * std::f32::consts::TAU).sin() * vol * decay * env_start);
    }
    out
}

/// Subtle pulse (amplitude-modulated sine).
pub fn pulse(freq: f32, dur: f32, vol: f32) -> Vec<f32> {
    let n = (SAMPLE_RATE as f32 * dur) as usize;
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let t = i as f32 / SAMPLE_RATE as f32;
        let env = envelope(t, dur);
        let am = 0.5 + 0.5 * (t * 8.0 * std::f32::consts::TAU).sin(); // 8 Hz tremolo
        out.push((t * freq * std::f32::consts::TAU).sin() * vol * env * am);
    }
    out
}

/// Soft pad: layered detuned sines for warmth.
pub fn pad(freq: f32, dur: f32, vol: f32) -> Vec<f32> {
    let detune = [0.995, 1.0, 1.005];
    let n = (SAMPLE_RATE as f32 * dur) as usize;
    let mut out = vec![0.0f32; n];
    for &d in &detune {
        let f = freq * d;
        for i in 0..n {
            let t = i as f32 / SAMPLE_RATE as f32;
            let env = envelope(t, dur);
            out[i] += (t * f * std::f32::consts::TAU).sin() * vol * env / detune.len() as f32;
        }
    }
    out
}

// ── Helpers ──────────────────────────────────────────────────────────

/// Smooth fade-in / fade-out envelope to avoid clicks.
fn envelope(t: f32, dur: f32) -> f32 {
    if t < FADE_SECS {
        t / FADE_SECS
    } else if t > dur - FADE_SECS {
        ((dur - t) / FADE_SECS).max(0.0)
    } else {
        1.0
    }
}

/// Convert raw f32 samples into a `SynthSound`.
pub fn make_sound(samples: Vec<f32>) -> SynthSound {
    SynthSound { samples: samples.into() }
}

// ── SoundSettings resource ───────────────────────────────────────────

/// Volume settings for the sound system.
#[derive(Resource, Clone)]
pub struct SoundSettings {
    pub master_volume: f32,
    pub sfx_volume: f32,
    pub muted: bool,
}

impl Default for SoundSettings {
    fn default() -> Self {
        Self { master_volume: 0.7, sfx_volume: 0.8, muted: false }
    }
}

// ── Plugin ───────────────────────────────────────────────────────────

/// Bevy plugin that registers the procedural sound system.
pub struct SoundPlugin;

impl Plugin for SoundPlugin {
    fn build(&self, app: &mut App) {
        app.add_audio_source::<SynthSound>();
        app.init_resource::<SoundSettings>();
        app.add_systems(Startup, setup_sound_palette);
    }
}
