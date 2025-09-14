use wasm_bindgen::prelude::*;
use crate::cpu::Cpu;
use crate::ppu::Ppu;
use crate::apu::Apu;
use crate::memory::Memory;
use crate::utils;

#[wasm_bindgen]
pub struct ZebratronSystem {
    cpu: Cpu,
    ppu: Ppu,
    apu: Apu,
    memory: Memory,
    running: bool,
    frame_ready: bool,
}

#[wasm_bindgen]
impl ZebratronSystem {
    #[wasm_bindgen(constructor)]
    pub fn new() -> ZebratronSystem {
        utils::set_panic_hook();

        ZebratronSystem {
            cpu: Cpu::new(),
            ppu: Ppu::new(),
            apu: Apu::new(),
            memory: Memory::new(),
            running: false,
            frame_ready: false,
        }
    }

    pub fn load_cartridge(&mut self, rom_data: &[u8]) -> bool {
        if rom_data.len() == 0 {
            return false;
        }

        self.memory.load_cartridge(rom_data);
        self.reset();
        true
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
        self.running = false;
        self.frame_ready = false;
    }

    pub fn start(&mut self) {
        self.running = true;
    }

    pub fn stop(&mut self) {
        self.running = false;
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn step_frame(&mut self) -> bool {
        if !self.running {
            return false;
        }

        self.frame_ready = false;

        // Run CPU and PPU until a frame is complete
        let mut cycles_this_frame = 0;
        let max_cycles_per_frame = 29780; // Approximate cycles for 60fps

        while cycles_this_frame < max_cycles_per_frame && !self.frame_ready {
            // Step CPU
            let cpu_cycles = self.cpu.step();

            // Step PPU (PPU runs 3x faster than CPU)
            for _ in 0..(cpu_cycles * 3) {
                if self.ppu.step(&self.memory) {
                    self.frame_ready = true;
                    break;
                }
            }

            // Step APU
            self.apu.step();

            cycles_this_frame += cpu_cycles as u32;
        }

        if self.frame_ready {
            self.ppu.render_frame(&self.memory);
        }

        self.frame_ready
    }

    // Graphics interface
    pub fn get_screen_buffer(&self) -> js_sys::Uint8Array {
        self.ppu.get_screen_buffer()
    }

    pub fn get_screen_width(&self) -> u32 {
        self.ppu.get_screen_width()
    }

    pub fn get_screen_height(&self) -> u32 {
        self.ppu.get_screen_height()
    }

    // Audio interface
    pub fn generate_audio_sample(&mut self) -> f32 {
        self.apu.generate_sample()
    }

    pub fn set_master_volume(&mut self, volume: f32) {
        self.apu.set_master_volume(volume);
    }

    // Debug interface
    pub fn get_cpu_state(&self) -> JsValue {
        let state = serde_json::json!({
            "a": self.cpu.a,
            "x": self.cpu.x,
            "y": self.cpu.y,
            "sp": self.cpu.sp,
            "pc": self.cpu.pc,
            "status": self.cpu.status,
            "cycles": self.cpu.cycles
        });
        serde_wasm_bindgen::to_value(&state).unwrap()
    }

    pub fn read_memory(&self, address: u16) -> u8 {
        self.memory.read_byte(address)
    }

    pub fn write_memory(&mut self, address: u16, value: u8) {
        self.memory.write_byte(address, value);
    }

    // Input interface
    pub fn handle_input(&mut self, up: bool, down: bool, left: bool, right: bool) {
        self.ppu.handle_input(up, down, left, right);
    }

    // Color test demo interface
    pub fn toggle_color_test(&mut self) {
        self.ppu.toggle_color_test_mode();
    }

    // Sound test methods
    pub fn enter_sound_test_mode(&mut self) {
        self.apu.enter_sound_test_mode();
    }

    pub fn exit_sound_test_mode(&mut self) {
        self.apu.exit_sound_test_mode();
    }

    pub fn sound_test_change_waveform(&mut self, waveform: u8) {
        self.apu.sound_test_change_waveform(waveform);
    }

    pub fn sound_test_change_note(&mut self, note: u8) {
        self.apu.sound_test_change_note(note);
    }

    pub fn sound_test_set_pulse_width(&mut self, width: f32) {
        self.apu.sound_test_set_pulse_width(width);
    }

    pub fn get_current_waveform(&self) -> u8 {
        self.apu.get_current_waveform()
    }

    pub fn get_current_note(&self) -> u8 {
        self.apu.get_current_note()
    }

    pub fn is_sound_test_mode(&self) -> bool {
        self.apu.is_sound_test_mode()
    }
}