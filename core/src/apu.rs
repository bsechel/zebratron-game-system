use wasm_bindgen::prelude::*;
use std::f32::consts::PI;

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
}

struct PulseChannel {
    enabled: bool,
    frequency: f32,
    duty_cycle: u8,
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
}

#[wasm_bindgen]
impl Apu {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Apu {
        Apu {
            pulse1: PulseChannel {
                enabled: false,
                frequency: 440.0,
                duty_cycle: 0,
                volume: 0,
                phase: 0.0,
            },
            pulse2: PulseChannel {
                enabled: false,
                frequency: 440.0,
                duty_cycle: 0,
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
            },
            master_volume: 0.5,
            sample_rate: 44100.0,
            frame_counter: 0,
            sound_test_mode: false,
            current_note: 69, // A4 = 440Hz
            current_waveform: 0,
        }
    }

    pub fn step(&mut self) {
        self.frame_counter += 1;
        // TODO: Implement proper frame sequencer timing
    }

    pub fn generate_sample(&mut self) -> f32 {
        let mut sample = 0.0;

        if self.sound_test_mode {
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

        sample * self.master_volume
    }

    fn generate_pulse_sample(channel: &mut PulseChannel, sample_rate: f32) -> f32 {
        let duty_table = [0.125, 0.25, 0.5, 0.75];
        let duty_threshold = duty_table[channel.duty_cycle as usize];

        channel.phase += channel.frequency / sample_rate;
        if channel.phase >= 1.0 {
            channel.phase -= 1.0;
        }

        let amplitude = if channel.phase < duty_threshold { 1.0 } else { -1.0 };
        amplitude * (channel.volume as f32 / 15.0)
    }

    fn generate_triangle_sample(channel: &mut TriangleChannel, sample_rate: f32) -> f32 {
        channel.phase += channel.frequency / sample_rate;
        if channel.phase >= 1.0 {
            channel.phase -= 1.0;
        }

        // Triangle wave: -1 to 1 and back
        let amplitude = if channel.phase < 0.5 {
            4.0 * channel.phase - 1.0
        } else {
            3.0 - 4.0 * channel.phase
        };

        amplitude * 0.5 // Triangle is quieter than pulse
    }

    fn generate_noise_sample(channel: &mut NoiseChannel) -> f32 {
        // Simple LFSR-based noise generation
        let feedback = ((channel.shift_register & 1) ^ ((channel.shift_register >> 1) & 1)) != 0;
        channel.shift_register >>= 1;
        if feedback {
            channel.shift_register |= 0x4000;
        }

        let amplitude = if (channel.shift_register & 1) != 0 { 1.0 } else { -1.0 };
        amplitude * (channel.volume as f32 / 15.0) * 0.5
    }

    // Register write methods (will be called by CPU when writing to APU registers)
    pub fn write_pulse1_register(&mut self, register: u8, value: u8) {
        match register {
            0 => {
                self.pulse1.duty_cycle = (value >> 6) & 3;
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

    fn generate_digital_oscillator_sample(osc: &mut DigitalOscillator, sample_rate: f32) -> f32 {
        let effective_freq = osc.frequency * (1.0 + osc.detune);
        osc.phase += effective_freq / sample_rate;

        // Keep phase in 0.0 to 1.0 range
        while osc.phase >= 1.0 {
            osc.phase -= 1.0;
        }

        let sample = match osc.waveform {
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

        sample * osc.volume
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

    pub fn set_master_volume(&mut self, volume: f32) {
        self.master_volume = volume.clamp(0.0, 1.0);
    }
}