use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Apu {
    // Audio channels
    pulse1: PulseChannel,
    pulse2: PulseChannel,
    triangle: TriangleChannel,
    noise: NoiseChannel,

    // Global audio settings
    master_volume: f32,
    sample_rate: f32,

    // Frame counter for timing
    frame_counter: u32,
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
            master_volume: 0.5,
            sample_rate: 44100.0,
            frame_counter: 0,
        }
    }

    pub fn step(&mut self) {
        self.frame_counter += 1;
        // TODO: Implement proper frame sequencer timing
    }

    pub fn generate_sample(&mut self) -> f32 {
        let mut sample = 0.0;

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

    pub fn set_master_volume(&mut self, volume: f32) {
        self.master_volume = volume.clamp(0.0, 1.0);
    }
}