use wasm_bindgen::prelude::*;
use std::f32::consts::PI;
use std::collections::HashMap;
use crate::laugh_sample::{LAUGH_SAMPLE_RETRO_SAMPLE_DATA, LAUGH_SAMPLE_RETRO_SAMPLE_RATE};

#[wasm_bindgen]
pub struct Apu {
    // Audio channels
    pulse1: PulseChannel,
    pulse2: PulseChannel,
    triangle: TriangleChannel,
    noise: NoiseChannel,

    // New digital oscillator for sound test
    test_osc: DigitalOscillator,

    // Global audio settings
    master_volume: f32,
    sample_rate: f32,

    // Frame counter for timing
    frame_counter: u32,

    // Sound test mode
    sound_test_mode: bool,
    current_note: u8,  // MIDI note number
    current_waveform: u8, // 0=pulse, 1=saw, 2=triangle, 3=sine, 4=noise

    // Demo melody sequencer
    melody_enabled: bool,
    melody_step: usize,
    melody_timer: f32,
    melody_tempo: f32,  // Steps per second
    melody_notes: [u8; 16], // MIDI notes for the melody

    // Sound effect system
    sfx_active: bool,
    sfx_timer: f32,
    sfx_duration: f32,
    sfx_start_note: u8,
    sfx_end_note: u8,
    sfx_waveform: u8,

    // Voice synthesis system for chip-tune voice effects
    voice_active: bool,
    voice_type: u8,        // 0=laughter, 1=gasp, 2=grunt
    voice_timer: f32,
    voice_stage: u8,       // Current stage of multi-part effect
    voice_stage_timer: f32,
    voice_oscillator: DigitalOscillator,

    // Polyphonic synthesizer for Z-Synth
    synth_oscillators: HashMap<u32, DigitalOscillator>, // MIDI note -> oscillator
    synth_enabled: bool,
    
    // Global filter settings for Z-Synth
    global_filter_enabled: bool,
    global_filter_type: u8,
    global_filter_cutoff: f32,
    global_filter_resonance: f32,
    
    // SID-style 3-voice synthesizer for games
    sid_voice1: DigitalOscillator,
    sid_voice2: DigitalOscillator,
    sid_voice3: DigitalOscillator,
    sid_enabled: bool,
    sid_volume: f32,
    poly_volume: f32,
    
    // Sample playback system for short audio clips
    sample_active: bool,
    sample_data: Option<&'static [u8]>, // Pointer to sample data
    sample_data_rate: u32,              // Sample rate of the data
    sample_position: f32,               // Current playback position (fractional)
    sample_volume: f32,                 // Volume for sample playback
}

struct PulseChannel {
    enabled: bool,
    frequency: f32,
    pulse_width: u8,
    volume: u8,
    phase: f32,
}

struct TriangleChannel {
    enabled: bool,
    frequency: f32,
    phase: f32,
}

struct NoiseChannel {
    enabled: bool,
    volume: u8,
    period: u16,
    shift_register: u16,
}

struct DigitalOscillator {
    enabled: bool,
    frequency: f32,
    waveform: u8,        // 0=pulse, 1=saw, 2=triangle, 3=sine, 4=noise
    phase: f32,
    pulse_width: f32,    // For pulse wave (0.0 to 1.0)
    volume: f32,
    detune: f32,         // Fine tuning offset
    lfsr: u16,          // For noise generation
    filter: ResonantFilter, // SID-style resonant filter
    delay: DigitalDelay,    // Digital delay effect
}

#[derive(Clone)]
struct ResonantFilter {
    enabled: bool,
    filter_type: u8,     // 0=lowpass, 1=highpass, 2=bandpass, 3=notch
    cutoff: f32,         // 0.0 to 1.0 (maps to 30Hz - 20kHz)
    resonance: f32,      // 0.0 to 1.0 (0.7+ starts self-oscillation)

    // Filter state variables (biquad implementation)
    x1: f32, x2: f32,    // Input delay line
    y1: f32, y2: f32,    // Output delay line

    // Filter coefficients (calculated from cutoff/resonance)
    a0: f32, a1: f32, a2: f32,
    b1: f32, b2: f32,
}

#[derive(Clone)]
struct DigitalDelay {
    enabled: bool,
    delay_time: f32,     // 0.0 to 1.0 (maps to 0ms - 1000ms)
    feedback: f32,       // 0.0 to 0.95 (0.95+ = infinite feedback)
    mix: f32,           // 0.0 = dry only, 1.0 = wet only, 0.5 = balanced

    // Delay buffer (circular buffer)
    buffer: Vec<f32>,
    buffer_size: usize,
    write_pos: usize,
    read_pos: usize,

    // Low-pass filter for analog-style delay character
    feedback_filter: f32, // Simple one-pole lowpass
}

#[wasm_bindgen]
impl Apu {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Apu {
        Apu {
            pulse1: PulseChannel {
                enabled: false,
                frequency: 440.0,
                pulse_width: 0,
                volume: 0,
                phase: 0.0,
            },
            pulse2: PulseChannel {
                enabled: false,
                frequency: 440.0,
                pulse_width: 0,
                volume: 0,
                phase: 0.0,
            },
            triangle: TriangleChannel {
                enabled: false,
                frequency: 220.0,
                phase: 0.0,
            },
            noise: NoiseChannel {
                enabled: false,
                volume: 0,
                period: 1,
                shift_register: 1,
            },
            test_osc: DigitalOscillator {
                enabled: false,
                frequency: 440.0,
                waveform: 0,
                phase: 0.0,
                pulse_width: 0.5,
                volume: 0.7,
                detune: 0.0,
                lfsr: 0x7FFF,
                filter: ResonantFilter {
                    enabled: true,
                    filter_type: 0,  // Lowpass (classic SID)
                    cutoff: 0.8,     // Start fairly open
                    resonance: 0.1,  // Mild resonance to start

                    // Initialize filter state
                    x1: 0.0, x2: 0.0,
                    y1: 0.0, y2: 0.0,

                    // Coefficients will be calculated
                    a0: 1.0, a1: 0.0, a2: 0.0,
                    b1: 0.0, b2: 0.0,
                },
                delay: DigitalDelay {
                    enabled: false,
                    delay_time: 0.3,        // 300ms default
                    feedback: 0.3,          // 30% feedback
                    mix: 0.25,             // 25% wet signal

                    // Initialize delay buffer (1 second max at 44.1kHz)
                    buffer: vec![0.0; 44100],
                    buffer_size: 44100,
                    write_pos: 0,
                    read_pos: 0,

                    feedback_filter: 0.0,
                },
            },
            master_volume: 0.5,
            sample_rate: 44100.0,
            frame_counter: 0,
            sound_test_mode: false,
            current_note: 69, // A4 = 440Hz
            current_waveform: 0,

            // Initialize demo melody (Russian-style minor melody)
            melody_enabled: false,
            melody_step: 0,
            melody_timer: 0.0,
            melody_tempo: 3.0, // Moderate tempo - 3 notes per second
            // Haunting Russian-style melody in D minor
            // D E♭ F G A♭ B♭ C D (D natural minor scale)
            melody_notes: [62, 65, 67, 62, 0, 65, 67, 70,   // D F A D rest F A B♭
                          72, 70, 67, 65, 62, 0, 62, 0],   // C B♭ A F D rest D rest

            // Initialize sound effects
            sfx_active: false,
            sfx_timer: 0.0,
            sfx_duration: 0.0,
            sfx_start_note: 60,
            sfx_end_note: 60,
            sfx_waveform: 0,

            // Initialize voice synthesis system
            voice_active: false,
            voice_type: 0,
            voice_timer: 0.0,
            voice_stage: 0,
            voice_stage_timer: 0.0,
            voice_oscillator: DigitalOscillator {
                enabled: false,
                frequency: 300.0,
                volume: 0.3,
                waveform: 0, // Pulse wave for voice
                phase: 0.0,
                pulse_width: 0.25, // Thin pulse for crisp voice sound
                detune: 0.0,
                lfsr: 0x7FFF,
                filter: ResonantFilter {
                    enabled: false,
                    filter_type: 0,
                    cutoff: 0.5,
                    resonance: 0.1,
                    x1: 0.0, x2: 0.0,
                    y1: 0.0, y2: 0.0,
                    a0: 1.0, a1: 0.0, a2: 0.0,
                    b1: 0.0, b2: 0.0,
                },
                delay: DigitalDelay {
                    enabled: false,
                    delay_time: 0.0,
                    feedback: 0.0,
                    mix: 0.0,
                    buffer: vec![0.0; 44100],
                    buffer_size: 44100,
                    write_pos: 0,
                    read_pos: 0,
                    feedback_filter: 0.0,
                },
            },

            // Initialize polyphonic synthesizer
            synth_oscillators: HashMap::new(),
            synth_enabled: false,
            
            // Initialize global filter settings
            global_filter_enabled: false,
            global_filter_type: 0, // Low pass
            global_filter_cutoff: 1000.0, // Hz
            global_filter_resonance: 0.5,
            
            // Initialize SID-style voices
            sid_voice1: DigitalOscillator {
                enabled: false,
                frequency: 440.0,
                waveform: 0, // Pulse wave
                phase: 0.0,
                pulse_width: 0.5,
                volume: 0.7,
                detune: 0.0,
                lfsr: 0x7FFF,
                filter: ResonantFilter {
                    enabled: false,
                    filter_type: 0,
                    cutoff: 0.8,
                    resonance: 0.2,
                    x1: 0.0, x2: 0.0,
                    y1: 0.0, y2: 0.0,
                    a0: 1.0, a1: 0.0, a2: 0.0,
                    b1: 0.0, b2: 0.0,
                },
                delay: DigitalDelay {
                    enabled: false,
                    delay_time: 0.3,
                    feedback: 0.4,
                    mix: 0.2,
                    buffer: vec![0.0; 44100],
                    buffer_size: 44100,
                    write_pos: 0,
                    read_pos: 0,
                    feedback_filter: 0.0,
                },
            },
            sid_voice2: DigitalOscillator {
                enabled: false,
                frequency: 440.0,
                waveform: 1, // Sawtooth wave
                phase: 0.0,
                pulse_width: 0.5,
                volume: 0.7,
                detune: 0.0,
                lfsr: 0x7FFF,
                filter: ResonantFilter {
                    enabled: false,
                    filter_type: 0,
                    cutoff: 0.8,
                    resonance: 0.2,
                    x1: 0.0, x2: 0.0,
                    y1: 0.0, y2: 0.0,
                    a0: 1.0, a1: 0.0, a2: 0.0,
                    b1: 0.0, b2: 0.0,
                },
                delay: DigitalDelay {
                    enabled: false,
                    delay_time: 0.3,
                    feedback: 0.4,
                    mix: 0.2,
                    buffer: vec![0.0; 44100],
                    buffer_size: 44100,
                    write_pos: 0,
                    read_pos: 0,
                    feedback_filter: 0.0,
                },
            },
            sid_voice3: DigitalOscillator {
                enabled: false,
                frequency: 440.0,
                waveform: 2, // Triangle wave
                phase: 0.0,
                pulse_width: 0.5,
                volume: 0.7,
                detune: 0.0,
                lfsr: 0x7FFF,
                filter: ResonantFilter {
                    enabled: false,
                    filter_type: 0,
                    cutoff: 0.8,
                    resonance: 0.2,
                    x1: 0.0, x2: 0.0,
                    y1: 0.0, y2: 0.0,
                    a0: 1.0, a1: 0.0, a2: 0.0,
                    b1: 0.0, b2: 0.0,
                },
                delay: DigitalDelay {
                    enabled: false,
                    delay_time: 0.3,
                    feedback: 0.4,
                    mix: 0.2,
                    buffer: vec![0.0; 44100],
                    buffer_size: 44100,
                    write_pos: 0,
                    read_pos: 0,
                    feedback_filter: 0.0,
                },
            },
            sid_enabled: false,
            sid_volume: 0.8,
            poly_volume: 0.8,
            
            // Sample playback initialization
            sample_active: false,
            sample_data: None,
            sample_data_rate: 5512,
            sample_position: 0.0,
            sample_volume: 0.8,
        }
    }

    pub fn step(&mut self) {
        self.frame_counter += 1;

        // Update melody sequencer if enabled
        if self.melody_enabled {
            // Advance melody timer (called once per CPU cycle, ~29780 times per frame at 60fps)
            self.melody_timer += 1.0 / (29780.0 * 60.0); // Actual step rate

            // Check if it's time for next melody step
            let step_duration = 1.0 / self.melody_tempo;
            if self.melody_timer >= step_duration {
                self.melody_timer = 0.0;

                // Move to next melody step
                self.melody_step = (self.melody_step + 1) % self.melody_notes.len();

                // Get the new note (0 = rest/silence)
                let note = self.melody_notes[self.melody_step];
                if note > 0 {
                    self.current_note = note;
                    self.test_osc.frequency = Self::midi_to_frequency(note);
                    self.test_osc.enabled = true;
                } else {
                    // Rest - disable oscillator briefly
                    self.test_osc.enabled = false;
                }
            }
        }

        // Update sound effect if active
        if self.sfx_active {
            // Advance sound effect timer
            self.sfx_timer += 1.0 / (29780.0 * 60.0); // Same rate as melody timer

            // Calculate progress (0.0 to 1.0)
            let progress = (self.sfx_timer / self.sfx_duration).min(1.0);

            if progress >= 1.0 {
                // Sound effect finished
                self.sfx_active = false;
                if self.sound_test_mode && !self.melody_enabled {
                    // Return to manual control
                    self.test_osc.frequency = Self::midi_to_frequency(self.current_note);
                } else {
                    // Disable oscillator if not in sound test mode
                    self.test_osc.enabled = false;
                }
            } else {
                // Interpolate between start and end note
                let current_note_float = self.sfx_start_note as f32 +
                    (self.sfx_end_note as f32 - self.sfx_start_note as f32) * progress;
                let current_freq = Self::midi_to_frequency(current_note_float as u8);

                // Apply to test oscillator
                self.test_osc.frequency = current_freq;
                self.test_osc.waveform = self.sfx_waveform;
                self.test_osc.enabled = true;
            }
        }

        // Update voice effect if active
        if self.voice_active {
            // Advance voice timer
            self.voice_timer += 1.0 / (29780.0 * 60.0);
            self.voice_stage_timer += 1.0 / (29780.0 * 60.0);
            
            // Update voice effect based on type and stage
            match self.voice_type {
                0 => self.update_laughter_effect(),
                1 => self.update_gasp_effect(), 
                2 => self.update_grunt_effect(),
                _ => self.voice_active = false,
            }
        }

        // TODO: Implement proper frame sequencer timing for other channels
    }

    pub fn generate_sample(&mut self) -> f32 {
        let mut sample = 0.0;

        // Voice effects (highest priority)
        if self.voice_active && self.voice_oscillator.enabled {
            sample += Self::generate_digital_oscillator_sample(&mut self.voice_oscillator, self.sample_rate);
        }
        
        // Sample playback (also high priority)
        let dt = 1.0 / self.sample_rate;
        sample += self.update_sample_playback(dt);
        
        // Always check for sound effects first
        if self.sfx_active && self.test_osc.enabled {
            sample += Self::generate_digital_oscillator_sample(&mut self.test_osc, self.sample_rate);
        } else if self.sound_test_mode {
            // In sound test mode, only use the test oscillator
            if self.test_osc.enabled {
                sample += Self::generate_digital_oscillator_sample(&mut self.test_osc, self.sample_rate);
            }
        } else {
            // Normal game mode - use all channels
            // Generate pulse channel 1
            if self.pulse1.enabled {
                sample += Self::generate_pulse_sample(&mut self.pulse1, self.sample_rate);
            }

            // Generate pulse channel 2
            if self.pulse2.enabled {
                sample += Self::generate_pulse_sample(&mut self.pulse2, self.sample_rate);
            }

            // Generate triangle channel
            if self.triangle.enabled {
                sample += Self::generate_triangle_sample(&mut self.triangle, self.sample_rate);
            }

            // Generate noise channel
            if self.noise.enabled {
                sample += Self::generate_noise_sample(&mut self.noise);
            }
        }

        // Generate polyphonic synthesizer (always active when notes are playing)
        if self.synth_enabled && !self.synth_oscillators.is_empty() {
            let mut poly_sample = 0.0;
            for osc in self.synth_oscillators.values_mut() {
                if osc.enabled {
                    poly_sample += Self::generate_digital_oscillator_sample(osc, self.sample_rate);
                }
            }
            sample += poly_sample * self.poly_volume;
        }

        // Generate SID-style 3-voice synthesizer (for games) - only if voices are active
        if self.sid_enabled && (self.sid_voice1.enabled || self.sid_voice2.enabled || self.sid_voice3.enabled) {
            let mut sid_sample = 0.0;
            if self.sid_voice1.enabled {
                sid_sample += Self::generate_digital_oscillator_sample(&mut self.sid_voice1, self.sample_rate);
            }
            if self.sid_voice2.enabled {
                sid_sample += Self::generate_digital_oscillator_sample(&mut self.sid_voice2, self.sample_rate);
            }
            if self.sid_voice3.enabled {
                sid_sample += Self::generate_digital_oscillator_sample(&mut self.sid_voice3, self.sample_rate);
            }
            sample += sid_sample * self.sid_volume;
        }

        sample * self.master_volume
    }

    fn generate_pulse_sample(channel: &mut PulseChannel, sample_rate: f32) -> f32 {
        let duty_table = [0.125, 0.25, 0.5, 0.75];
        let duty_threshold = duty_table[channel.pulse_width as usize];

        channel.phase += channel.frequency / sample_rate;
        if channel.phase >= 1.0 {
            channel.phase -= 1.0;
        }

        let volume = if channel.phase < duty_threshold { 1.0 } else { -1.0 };
        volume * (channel.volume as f32 / 15.0)
    }

    fn generate_triangle_sample(channel: &mut TriangleChannel, sample_rate: f32) -> f32 {
        channel.phase += channel.frequency / sample_rate;
        if channel.phase >= 1.0 {
            channel.phase -= 1.0;
        }

        // Triangle wave: -1 to 1 and back
        let volume = if channel.phase < 0.5 {
            4.0 * channel.phase - 1.0
        } else {
            3.0 - 4.0 * channel.phase
        };

        volume * 0.5 // Triangle is quieter than pulse
    }

    fn generate_noise_sample(channel: &mut NoiseChannel) -> f32 {
        // Simple LFSR-based noise generation
        let feedback = ((channel.shift_register & 1) ^ ((channel.shift_register >> 1) & 1)) != 0;
        channel.shift_register >>= 1;
        if feedback {
            channel.shift_register |= 0x4000;
        }

        let volume = if (channel.shift_register & 1) != 0 { 1.0 } else { -1.0 };
        volume * (channel.volume as f32 / 15.0) * 0.5
    }

    // Register write methods (will be called by CPU when writing to APU registers)
    pub fn write_pulse1_register(&mut self, register: u8, value: u8) {
        match register {
            0 => {
                self.pulse1.pulse_width = (value >> 6) & 3;
                self.pulse1.volume = value & 15;
            }
            1 => {
                // Sweep register (TODO: implement sweep)
            }
            2 => {
                // Frequency low byte
                let freq_raw = (self.pulse1.frequency as u16 & 0x700) | value as u16;
                self.pulse1.frequency = 1789773.0 / (16.0 * (freq_raw as f32 + 1.0));
            }
            3 => {
                // Frequency high byte + length
                let freq_raw = ((value as u16 & 7) << 8) | (self.pulse1.frequency as u16 & 0xFF);
                self.pulse1.frequency = 1789773.0 / (16.0 * (freq_raw as f32 + 1.0));
                self.pulse1.enabled = true;
                self.pulse1.phase = 0.0;
            }
            _ => {}
        }
    }

    fn update_filter_coefficients(filter: &mut ResonantFilter, sample_rate: f32) {
        // Calculate filter coefficients from cutoff and resonance
        // Classic SID frequency range: ~30Hz to ~20kHz
        let min_freq = 30.0;
        let max_freq = 20000.0;
        let freq = min_freq + filter.cutoff * (max_freq - min_freq);

        let omega = 2.0 * PI * freq / sample_rate;
        let sin_omega = omega.sin();
        let cos_omega = omega.cos();

        // Resonance: 0.5 = no resonance, 10.0+ = self-oscillation
        let q = 0.5 + filter.resonance * 15.0;
        let alpha = sin_omega / (2.0 * q);

        match filter.filter_type {
            0 => {
                // Lowpass (classic SID sound)
                let norm = 1.0 + alpha;
                filter.a0 = (1.0 - cos_omega) * 0.5 / norm;
                filter.a1 = (1.0 - cos_omega) / norm;
                filter.a2 = (1.0 - cos_omega) * 0.5 / norm;
                filter.b1 = -2.0 * cos_omega / norm;
                filter.b2 = (1.0 - alpha) / norm;
            },
            1 => {
                // Highpass
                let norm = 1.0 + alpha;
                filter.a0 = (1.0 + cos_omega) * 0.5 / norm;
                filter.a1 = -(1.0 + cos_omega) / norm;
                filter.a2 = (1.0 + cos_omega) * 0.5 / norm;
                filter.b1 = -2.0 * cos_omega / norm;
                filter.b2 = (1.0 - alpha) / norm;
            },
            2 => {
                // Bandpass
                let norm = 1.0 + alpha;
                filter.a0 = sin_omega * 0.5 / norm;
                filter.a1 = 0.0;
                filter.a2 = -sin_omega * 0.5 / norm;
                filter.b1 = -2.0 * cos_omega / norm;
                filter.b2 = (1.0 - alpha) / norm;
            },
            _ => {
                // Notch (band-reject)
                let norm = 1.0 + alpha;
                filter.a0 = 1.0 / norm;
                filter.a1 = -2.0 * cos_omega / norm;
                filter.a2 = 1.0 / norm;
                filter.b1 = -2.0 * cos_omega / norm;
                filter.b2 = (1.0 - alpha) / norm;
            }
        }
    }

    fn apply_resonant_filter(filter: &mut ResonantFilter, input: f32) -> f32 {
        if !filter.enabled {
            return input;
        }

        // Biquad filter implementation
        let output = filter.a0 * input + filter.a1 * filter.x1 + filter.a2 * filter.x2
                    - filter.b1 * filter.y1 - filter.b2 * filter.y2;

        // Update delay lines
        filter.x2 = filter.x1;
        filter.x1 = input;
        filter.y2 = filter.y1;
        filter.y1 = output;

        // Soft clipping to prevent filter instability at high resonance
        output.clamp(-2.0, 2.0)
    }

    fn update_delay_buffer_positions(delay: &mut DigitalDelay, sample_rate: f32) {
        // Calculate delay time in samples (0ms to 1000ms)
        let delay_samples = (delay.delay_time * 1000.0 * sample_rate / 1000.0) as usize;
        let delay_samples = delay_samples.min(delay.buffer_size - 1).max(1);

        // Update read position (circular buffer)
        delay.read_pos = if delay.write_pos >= delay_samples {
            delay.write_pos - delay_samples
        } else {
            delay.buffer_size - (delay_samples - delay.write_pos)
        };
    }

    fn apply_digital_delay(delay: &mut DigitalDelay, input: f32, sample_rate: f32) -> f32 {
        if !delay.enabled {
            return input;
        }

        // Update buffer positions based on delay time
        Self::update_delay_buffer_positions(delay, sample_rate);

        // Read delayed sample
        let delayed_sample = delay.buffer[delay.read_pos];

        // Apply feedback with analog-style filtering
        // Simple one-pole lowpass: y[n] = a*x[n] + (1-a)*y[n-1]
        let filter_coeff = 0.8; // Darken the feedback (like analog tape)
        delay.feedback_filter = filter_coeff * delayed_sample + (1.0 - filter_coeff) * delay.feedback_filter;

        // Create feedback signal
        let feedback_signal = delay.feedback_filter * delay.feedback;

        // Write new sample to buffer (input + feedback)
        delay.buffer[delay.write_pos] = input + feedback_signal;

        // Advance write position (circular buffer)
        delay.write_pos = (delay.write_pos + 1) % delay.buffer_size;

        // Mix dry and wet signals
        let dry = input * (1.0 - delay.mix);
        let wet = delayed_sample * delay.mix;

        // Soft clipping to prevent digital distortion
        (dry + wet).clamp(-1.5, 1.5)
    }

    fn generate_digital_oscillator_sample(osc: &mut DigitalOscillator, sample_rate: f32) -> f32 {
        let effective_freq = osc.frequency * (1.0 + osc.detune);
        osc.phase += effective_freq / sample_rate;

        // Keep phase in 0.0 to 1.0 range
        while osc.phase >= 1.0 {
            osc.phase -= 1.0;
        }

        // Generate raw waveform
        let raw_sample = match osc.waveform {
            0 => {
                // Pulse wave (square with variable pulse width)
                if osc.phase < osc.pulse_width { 1.0 } else { -1.0 }
            },
            1 => {
                // Sawtooth wave
                2.0 * osc.phase - 1.0
            },
            2 => {
                // Triangle wave
                if osc.phase < 0.5 {
                    4.0 * osc.phase - 1.0
                } else {
                    3.0 - 4.0 * osc.phase
                }
            },
            3 => {
                // Sine wave
                (osc.phase * 2.0 * PI).sin()
            },
            4 => {
                // Digital noise (LFSR)
                let feedback = ((osc.lfsr & 1) ^ ((osc.lfsr >> 1) & 1)) != 0;
                osc.lfsr >>= 1;
                if feedback {
                    osc.lfsr |= 0x4000;
                }
                if (osc.lfsr & 1) != 0 { 1.0 } else { -1.0 }
            },
            _ => 0.0,
        };

        // Update filter coefficients if needed (for efficiency, could cache this)
        Self::update_filter_coefficients(&mut osc.filter, sample_rate);

        // Apply resonant filter to the raw waveform
        let filtered_sample = Self::apply_resonant_filter(&mut osc.filter, raw_sample);

        // Apply digital delay effect
        let delayed_sample = Self::apply_digital_delay(&mut osc.delay, filtered_sample, sample_rate);

        delayed_sample * osc.volume
    }

    // MIDI note to frequency conversion
    fn midi_to_frequency(note: u8) -> f32 {
        440.0 * 2.0_f32.powf((note as f32 - 69.0) / 12.0)
    }

    // Sound test control methods
    pub fn enter_sound_test_mode(&mut self) {
        self.sound_test_mode = true;
        self.test_osc.enabled = true;
        self.test_osc.frequency = Self::midi_to_frequency(self.current_note);
        self.test_osc.waveform = self.current_waveform;
    }

    pub fn exit_sound_test_mode(&mut self) {
        self.sound_test_mode = false;
        self.test_osc.enabled = false;
        self.melody_enabled = false; // Stop melody when exiting sound test
    }

    pub fn sound_test_change_waveform(&mut self, waveform: u8) {
        self.current_waveform = waveform.clamp(0, 4);
        self.test_osc.waveform = self.current_waveform;
    }

    pub fn sound_test_change_note(&mut self, note: u8) {
        self.current_note = note.clamp(21, 108); // Piano range A0 to C8
        self.test_osc.frequency = Self::midi_to_frequency(self.current_note);
    }

    pub fn sound_test_set_pulse_width(&mut self, width: f32) {
        self.test_osc.pulse_width = width.clamp(0.05, 0.95);
    }

    pub fn sound_test_set_detune(&mut self, detune: f32) {
        self.test_osc.detune = detune.clamp(-0.5, 0.5);
    }

    pub fn get_current_waveform(&self) -> u8 {
        self.current_waveform
    }

    pub fn get_current_note(&self) -> u8 {
        self.current_note
    }

    pub fn is_sound_test_mode(&self) -> bool {
        self.sound_test_mode
    }

    // Filter control methods (for test oscillator only - Z-Synth uses global filter methods at end of file)

    pub fn get_filter_cutoff(&self) -> f32 {
        self.test_osc.filter.cutoff
    }

    pub fn get_filter_resonance(&self) -> f32 {
        self.test_osc.filter.resonance
    }

    pub fn get_filter_type(&self) -> u8 {
        self.test_osc.filter.filter_type
    }

    // Delay control methods
    pub fn set_delay_enabled(&mut self, enabled: bool) {
        self.test_osc.delay.enabled = enabled;
    }

    pub fn set_delay_time(&mut self, delay_time: f32) {
        self.test_osc.delay.delay_time = delay_time.clamp(0.0, 1.0);
    }

    pub fn set_delay_feedback(&mut self, feedback: f32) {
        self.test_osc.delay.feedback = feedback.clamp(0.0, 0.95); // Prevent runaway feedback
    }

    pub fn set_delay_mix(&mut self, mix: f32) {
        self.test_osc.delay.mix = mix.clamp(0.0, 1.0);
    }

    pub fn get_delay_enabled(&self) -> bool {
        self.test_osc.delay.enabled
    }

    pub fn get_delay_time(&self) -> f32 {
        self.test_osc.delay.delay_time
    }

    pub fn get_delay_feedback(&self) -> f32 {
        self.test_osc.delay.feedback
    }

    pub fn get_delay_mix(&self) -> f32 {
        self.test_osc.delay.mix
    }

    // Demo melody control methods
    pub fn set_melody_enabled(&mut self, enabled: bool) {
        self.melody_enabled = enabled;
        if enabled {
            // Enable sound test mode for melody playback
            self.sound_test_mode = true;
            self.test_osc.enabled = true;
            self.test_osc.waveform = self.current_waveform;
            // Reset melody to beginning when enabling
            self.melody_step = 0;
            self.melody_timer = 0.0;
            // Start with first note
            let note = self.melody_notes[0];
            if note > 0 {
                self.current_note = note;
                self.test_osc.frequency = Self::midi_to_frequency(note);
            }
        } else {
            // Keep sound test mode active but use manual control
            if self.sound_test_mode {
                self.test_osc.frequency = Self::midi_to_frequency(self.current_note);
                self.test_osc.enabled = true;
            }
        }
    }

    pub fn get_melody_enabled(&self) -> bool {
        self.melody_enabled
    }

    pub fn set_melody_tempo(&mut self, tempo: f32) {
        self.melody_tempo = tempo.clamp(0.5, 4.0); // 0.5 to 4 steps per second
    }

    pub fn get_melody_tempo(&self) -> f32 {
        self.melody_tempo
    }

    // Sound effect methods
    pub fn play_sound_effect(&mut self, start_note: u8, end_note: u8, waveform: u8, duration: f32) {
        // Don't interrupt an already playing sound effect
        if self.sfx_active {
            return;
        }
        
        self.sfx_active = true;
        self.sfx_timer = 0.0;
        self.sfx_duration = duration;
        self.sfx_start_note = start_note;
        self.sfx_end_note = end_note;
        self.sfx_waveform = waveform;

        // Immediately set the starting frequency and waveform
        self.test_osc.frequency = Self::midi_to_frequency(start_note);
        self.test_osc.waveform = waveform;
        self.test_osc.enabled = true;
    }

    pub fn play_voice_effect(&mut self, voice_type: u8) {
        // Don't interrupt an already playing voice effect
        if self.voice_active {
            return;
        }
        
        self.voice_active = true;
        self.voice_type = voice_type;
        self.voice_timer = 0.0;
        self.voice_stage = 0;
        self.voice_stage_timer = 0.0;
        
        // Initialize the voice effect based on type
        match voice_type {
            0 => self.start_laughter_effect(),
            1 => self.start_gasp_effect(),
            2 => self.start_grunt_effect(),
            _ => self.voice_active = false, // Invalid type
        }
    }

    fn start_laughter_effect(&mut self) {
        // Laughter: "Ha-ha-ha" - 3 stage effect
        // Stage 0: "Ha" (300Hz, 100ms)
        self.voice_oscillator.enabled = true;
        self.voice_oscillator.frequency = 300.0;
        self.voice_oscillator.volume = 0.4;
        self.voice_oscillator.waveform = 0; // Pulse wave
        self.voice_oscillator.pulse_width = 0.25; // Thin pulse
    }

    fn start_gasp_effect(&mut self) {
        // Gasp: Sharp intake - noise + triangle sweep
        self.voice_oscillator.enabled = true;
        self.voice_oscillator.frequency = 150.0;
        self.voice_oscillator.volume = 0.3;
        self.voice_oscillator.waveform = 3; // Noise
    }

    fn start_grunt_effect(&mut self) {
        // Grunt: Low effort sound
        self.voice_oscillator.enabled = true;
        self.voice_oscillator.frequency = 80.0;
        self.voice_oscillator.volume = 0.5;
        self.voice_oscillator.waveform = 0; // Pulse wave
        self.voice_oscillator.pulse_width = 0.75; // Thick pulse for grunt
    }

    fn update_laughter_effect(&mut self) {
        // Laughter: "Ha-ha-ha" with gaps - 6 stages total
        // Stages 0,2,4: "Ha" sounds (1000ms each)
        // Stages 1,3,5: Silence gaps (500ms each)
        let stage_duration = if self.voice_stage % 2 == 0 { 1.0 } else { 0.5 }; // 1000ms ha, 500ms gap
        
        if self.voice_stage_timer >= stage_duration {
            self.voice_stage_timer = 0.0;
            self.voice_stage += 1;
            
            match self.voice_stage {
                1 => {
                    // Gap after first "ha"
                    self.voice_oscillator.enabled = false;
                },
                2 => {
                    // Second "ha" - higher pitch
                    self.voice_oscillator.enabled = true;
                    self.voice_oscillator.frequency = 400.0;
                    self.voice_oscillator.volume = 0.35;
                },
                3 => {
                    // Gap after second "ha"
                    self.voice_oscillator.enabled = false;
                },
                4 => {
                    // Third "ha" - middle pitch
                    self.voice_oscillator.enabled = true;
                    self.voice_oscillator.frequency = 350.0;
                    self.voice_oscillator.volume = 0.3;
                },
                5 => {
                    // Final gap
                    self.voice_oscillator.enabled = false;
                },
                _ => {
                    // Laughter complete
                    self.voice_active = false;
                    self.voice_oscillator.enabled = false;
                }
            }
        }
    }

    fn update_gasp_effect(&mut self) {
        // Gasp: 200ms effect with frequency sweep
        let duration = 0.2;
        let progress = (self.voice_timer / duration).min(1.0);
        
        if progress >= 1.0 {
            self.voice_active = false;
            self.voice_oscillator.enabled = false;
        } else {
            // Sweep from 150Hz to 800Hz (inhale effect)
            let freq = 150.0 + (800.0 - 150.0) * progress;
            self.voice_oscillator.frequency = freq;
            
            // Fade out volume
            self.voice_oscillator.volume = 0.3 * (1.0 - progress);
        }
    }

    fn update_grunt_effect(&mut self) {
        // Grunt: 150ms with slight pitch bend down
        let duration = 0.15;
        let progress = (self.voice_timer / duration).min(1.0);
        
        if progress >= 1.0 {
            self.voice_active = false;
            self.voice_oscillator.enabled = false;
        } else {
            // Slight pitch bend down (80Hz to 70Hz)
            let freq = 80.0 - 10.0 * progress;
            self.voice_oscillator.frequency = freq;
            
            // Maintain volume throughout
            self.voice_oscillator.volume = 0.5;
        }
    }

    pub fn set_master_volume(&mut self, volume: f32) {
        self.master_volume = volume.clamp(0.0, 1.0);
    }
    
    // Sample playback methods
    pub fn stop_sample(&mut self) {
        self.sample_active = false;
        self.sample_data = None;
        self.sample_position = 0.0;
    }
    
    pub fn set_sample_volume(&mut self, volume: f32) {
        self.sample_volume = volume.clamp(0.0, 1.0);
    }
    
    pub fn play_laugh_sample(&mut self) {
        self.sample_data = Some(LAUGH_SAMPLE_RETRO_SAMPLE_DATA);
        self.sample_data_rate = LAUGH_SAMPLE_RETRO_SAMPLE_RATE;
        self.sample_position = 0.0;
        self.sample_active = true;
    }
    
    fn update_sample_playback(&mut self, dt: f32) -> f32 {
        if !self.sample_active {
            return 0.0;
        }
        
        if let Some(data) = self.sample_data {
            let sample_index = self.sample_position as usize;
            
            // Check if we've reached the end of the sample
            if sample_index >= data.len() {
                self.sample_active = false;
                self.sample_data = None;
                return 0.0;
            }
            
            // Get current sample (convert from 8-bit unsigned to signed float)
            let sample_u8 = data[sample_index];
            let sample_f32 = (sample_u8 as f32 - 128.0) / 128.0; // Convert to -1.0 to 1.0 range
            
            // Advance position based on sample rate ratio
            // APU runs at 44.1kHz, sample data is at 5.5kHz, but we want proper pitch/duration balance
            // Using 0.75 to get closer to original pitch while keeping retro quality
            let advance_rate = (self.sample_data_rate as f32 / self.sample_rate) * 0.75;
            self.sample_position += advance_rate;
            
            return sample_f32 * self.sample_volume;
        }
        
        0.0
    }

    // Polyphonic synthesizer methods for Z-Synth
    pub fn synth_note_on(&mut self, note: u32) {
        if !self.synth_oscillators.contains_key(&note) {
            let mut osc = DigitalOscillator {
                enabled: true,
                frequency: Self::midi_to_frequency(note as u8),
                waveform: 0, // Start with pulse wave
                phase: 0.0,
                pulse_width: 0.5,
                volume: 0.3, // Lower volume for polyphony
                detune: 0.0,
                lfsr: 0x7FFF,
                filter: ResonantFilter {
                    enabled: self.global_filter_enabled,
                    filter_type: self.global_filter_type,
                    cutoff: (self.global_filter_cutoff / (self.sample_rate * 0.5)).min(1.0),
                    resonance: self.global_filter_resonance,
                    x1: 0.0, x2: 0.0,
                    y1: 0.0, y2: 0.0,
                    a0: 1.0, a1: 0.0, a2: 0.0,
                    b1: 0.0, b2: 0.0,
                },
                delay: DigitalDelay {
                    enabled: false,
                    delay_time: 0.3,
                    feedback: 0.4,
                    mix: 0.2,
                    buffer: vec![0.0; 44100], // 1 second buffer at 44.1kHz
                    buffer_size: 44100,
                    write_pos: 0,
                    read_pos: 0,
                    feedback_filter: 0.0,
                },
            };
            
            // Calculate filter coefficients for the new oscillator
            Self::update_filter_coefficients(&mut osc.filter, self.sample_rate);
            
            self.synth_oscillators.insert(note, osc);
        }
        self.synth_enabled = true;
    }

    pub fn synth_note_off(&mut self, note: u32) {
        self.synth_oscillators.remove(&note);
        if self.synth_oscillators.is_empty() {
            self.synth_enabled = false;
        }
    }

    pub fn set_synth_enabled(&mut self, enabled: bool) {
        self.synth_enabled = enabled;
        if !enabled {
            self.synth_oscillators.clear();
        }
    }

    pub fn get_synth_active_note_count(&self) -> usize {
        self.synth_oscillators.len()
    }
    
    // Global filter control methods for Z-Synth
    #[wasm_bindgen]
    pub fn set_filter_enabled(&mut self, enabled: bool) {
        self.global_filter_enabled = enabled;
        // Apply to all active synth oscillators
        for osc in self.synth_oscillators.values_mut() {
            osc.filter.enabled = enabled;
        }
    }
    
    #[wasm_bindgen]
    pub fn set_filter_type(&mut self, filter_type: u8) {
        self.global_filter_type = filter_type;
        // Apply to all active synth oscillators
        for osc in self.synth_oscillators.values_mut() {
            osc.filter.filter_type = filter_type;
            Self::update_filter_coefficients(&mut osc.filter, self.sample_rate);
        }
    }
    
    #[wasm_bindgen]
    pub fn set_filter_cutoff(&mut self, cutoff: f32) {
        self.global_filter_cutoff = cutoff;
        // Convert Hz to normalized cutoff (0.0 to 1.0)
        let normalized_cutoff = (cutoff / (self.sample_rate * 0.5)).min(1.0);
        // Apply to all active synth oscillators
        for osc in self.synth_oscillators.values_mut() {
            osc.filter.cutoff = normalized_cutoff;
            Self::update_filter_coefficients(&mut osc.filter, self.sample_rate);
        }
    }
    
    #[wasm_bindgen]
    pub fn set_filter_resonance(&mut self, resonance: f32) {
        self.global_filter_resonance = resonance;
        // Apply to all active synth oscillators
        for osc in self.synth_oscillators.values_mut() {
            osc.filter.resonance = resonance;
            Self::update_filter_coefficients(&mut osc.filter, self.sample_rate);
        }
    }
    
    // SID-style 3-voice API for game developers
    #[wasm_bindgen]
    pub fn sid_voice1_play_note(&mut self, note: u8, waveform: u8) {
        self.sid_voice1.frequency = Self::midi_to_frequency(note);
        self.sid_voice1.waveform = waveform.clamp(0, 4);
        self.sid_voice1.enabled = true;
        self.sid_enabled = true;
    }
    
    #[wasm_bindgen]
    pub fn sid_voice2_play_note(&mut self, note: u8, waveform: u8) {
        self.sid_voice2.frequency = Self::midi_to_frequency(note);
        self.sid_voice2.waveform = waveform.clamp(0, 4);
        self.sid_voice2.enabled = true;
        self.sid_enabled = true;
    }
    
    #[wasm_bindgen]
    pub fn sid_voice3_play_note(&mut self, note: u8, waveform: u8) {
        self.sid_voice3.frequency = Self::midi_to_frequency(note);
        self.sid_voice3.waveform = waveform.clamp(0, 4);
        self.sid_voice3.enabled = true;
        self.sid_enabled = true;
    }
    
    #[wasm_bindgen]
    pub fn sid_voice1_stop(&mut self) {
        self.sid_voice1.enabled = false;
        self.check_sid_enabled();
    }
    
    #[wasm_bindgen]
    pub fn sid_voice2_stop(&mut self) {
        self.sid_voice2.enabled = false;
        self.check_sid_enabled();
    }
    
    #[wasm_bindgen]
    pub fn sid_voice3_stop(&mut self) {
        self.sid_voice3.enabled = false;
        self.check_sid_enabled();
    }
    
    #[wasm_bindgen]
    pub fn sid_stop_all(&mut self) {
        self.sid_voice1.enabled = false;
        self.sid_voice2.enabled = false;
        self.sid_voice3.enabled = false;
        self.sid_enabled = false;
    }
    
    // Volume control for mixing SID and polyphonic layers
    #[wasm_bindgen]
    pub fn set_sid_volume(&mut self, volume: f32) {
        self.sid_volume = volume.clamp(0.0, 1.0);
    }
    
    #[wasm_bindgen]
    pub fn set_poly_volume(&mut self, volume: f32) {
        self.poly_volume = volume.clamp(0.0, 1.0);
    }
    
    // SID filter control (affects all 3 voices like real SID)
    #[wasm_bindgen]
    pub fn sid_set_filter_voices(&mut self, voice1: bool, voice2: bool, voice3: bool) {
        self.sid_voice1.filter.enabled = voice1;
        self.sid_voice2.filter.enabled = voice2;
        self.sid_voice3.filter.enabled = voice3;
    }
    
    #[wasm_bindgen]
    pub fn sid_set_filter_cutoff(&mut self, cutoff: f32) {
        let normalized_cutoff = (cutoff / (self.sample_rate * 0.5)).min(1.0);
        self.sid_voice1.filter.cutoff = normalized_cutoff;
        self.sid_voice2.filter.cutoff = normalized_cutoff;
        self.sid_voice3.filter.cutoff = normalized_cutoff;
        Self::update_filter_coefficients(&mut self.sid_voice1.filter, self.sample_rate);
        Self::update_filter_coefficients(&mut self.sid_voice2.filter, self.sample_rate);
        Self::update_filter_coefficients(&mut self.sid_voice3.filter, self.sample_rate);
    }
    
    #[wasm_bindgen]
    pub fn sid_set_filter_resonance(&mut self, resonance: f32) {
        let clamped_resonance = resonance.clamp(0.0, 10.0);
        self.sid_voice1.filter.resonance = clamped_resonance;
        self.sid_voice2.filter.resonance = clamped_resonance;
        self.sid_voice3.filter.resonance = clamped_resonance;
        Self::update_filter_coefficients(&mut self.sid_voice1.filter, self.sample_rate);
        Self::update_filter_coefficients(&mut self.sid_voice2.filter, self.sample_rate);
        Self::update_filter_coefficients(&mut self.sid_voice3.filter, self.sample_rate);
    }
    
    #[wasm_bindgen]
    pub fn sid_set_filter_type(&mut self, filter_type: u8) {
        let clamped_type = filter_type.clamp(0, 2);
        self.sid_voice1.filter.filter_type = clamped_type;
        self.sid_voice2.filter.filter_type = clamped_type;
        self.sid_voice3.filter.filter_type = clamped_type;
        Self::update_filter_coefficients(&mut self.sid_voice1.filter, self.sample_rate);
        Self::update_filter_coefficients(&mut self.sid_voice2.filter, self.sample_rate);
        Self::update_filter_coefficients(&mut self.sid_voice3.filter, self.sample_rate);
    }
    
    // Polyphonic layer API (enhanced Z-Synth access)
    #[wasm_bindgen]
    pub fn poly_play_chord(&mut self, notes: Vec<u8>) {
        // Stop all current notes and play new chord
        self.synth_oscillators.clear();
        for note in notes {
            self.synth_note_on(note as u32);
        }
    }
    
    #[wasm_bindgen]
    pub fn poly_play_note(&mut self, note: u8) {
        self.synth_note_on(note as u32);
    }
    
    #[wasm_bindgen]
    pub fn poly_stop_note(&mut self, note: u8) {
        self.synth_note_off(note as u32);
    }
    
    #[wasm_bindgen]
    pub fn poly_stop_all(&mut self) {
        self.synth_oscillators.clear();
        self.synth_enabled = false;
    }
    
    // Helper method to check if any SID voices are active
    fn check_sid_enabled(&mut self) {
        self.sid_enabled = self.sid_voice1.enabled || self.sid_voice2.enabled || self.sid_voice3.enabled;
    }
}