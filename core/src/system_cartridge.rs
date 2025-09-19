use wasm_bindgen::prelude::*;
use crate::cpu::Cpu;
use crate::ppu_clean::Ppu;
use crate::apu::Apu;
use crate::memory::Memory;
use crate::cartridge::HambertCartridge;
use crate::utils;

#[wasm_bindgen]
pub struct ZebratronCartridgeSystem {
    cpu: Cpu,
    ppu: Ppu,
    apu: Apu,
    memory: Memory,
    cartridge: Option<HambertCartridge>,
    running: bool,
    frame_ready: bool,
}

#[wasm_bindgen]
impl ZebratronCartridgeSystem {
    #[wasm_bindgen(constructor)]
    pub fn new() -> ZebratronCartridgeSystem {
        utils::set_panic_hook();

        ZebratronCartridgeSystem {
            cpu: Cpu::new(),
            ppu: Ppu::new(),
            apu: Apu::new(),
            memory: Memory::new(),
            cartridge: None,
            running: false,
            frame_ready: false,
        }
    }

    // Load the Hambert cartridge
    pub fn load_hambert_cartridge(&mut self) -> bool {
        let hambert = HambertCartridge::new();
        self.cartridge = Some(hambert);
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

    // Step one frame - handles cartridge update and PPU rendering
    pub fn step_frame(&mut self) -> bool {
        if !self.running {
            return false;
        }

        // Step PPU until a frame is complete (authentic timing)
        loop {
            let frame_complete = self.ppu.step(&self.memory);

            if frame_complete {
                self.frame_ready = true;

                // Update cartridge game logic every frame
                if let Some(ref mut cartridge) = self.cartridge {
                    cartridge.update_game(false, false, false, false); // No input here, input handled separately
                }

                // Sync cartridge data with PPU
                self.sync_cartridge_to_ppu();

                return true;
            }
        }
    }

    // Update cartridge game logic and sync with PPU
    pub fn handle_input(&mut self, up: bool, down: bool, left: bool, right: bool) {
        if let Some(ref mut cartridge) = self.cartridge {
            // Update cartridge with input
            cartridge.update_game(up, down, left, right);

            // Sync cartridge data with PPU
            self.sync_cartridge_to_ppu();
        }
    }

    fn sync_cartridge_to_ppu(&mut self) {
        if let Some(ref cartridge) = self.cartridge {
            // Update PPU scroll position based on cartridge camera
            let camera_x = cartridge.get_camera_x();
            let camera_y = cartridge.get_camera_y();
            self.ppu.set_scroll(camera_x, camera_y);

            // Clear existing sprites
            self.ppu.clear_sprites();

            // Add cartridge entities as sprites to PPU
            for i in 0..cartridge.get_entity_count() {
                if let Some(entity_data) = cartridge.get_entity_data(i) {
                    let x = js_sys::Reflect::get(&entity_data, &"x".into())
                        .unwrap()
                        .as_f64()
                        .unwrap_or(0.0) as f32;
                    let y = js_sys::Reflect::get(&entity_data, &"y".into())
                        .unwrap()
                        .as_f64()
                        .unwrap_or(0.0) as f32;
                    let sprite_id = js_sys::Reflect::get(&entity_data, &"sprite_id".into())
                        .unwrap()
                        .as_f64()
                        .unwrap_or(0.0) as u32;
                    let active = js_sys::Reflect::get(&entity_data, &"active".into())
                        .unwrap()
                        .as_bool()
                        .unwrap_or(false);

                    self.ppu.add_sprite(x, y, sprite_id, active);
                }
            }
        }
    }

    pub fn render(&mut self) {
        self.ppu.render();
    }

    pub fn get_screen_buffer(&self) -> js_sys::Uint8Array {
        let buffer = self.ppu.get_screen_buffer();
        js_sys::Uint8Array::from(&buffer[..])
    }

    // PPU control methods
    pub fn toggle_color_test(&mut self) {
        self.ppu.toggle_color_test();
    }

    pub fn get_color_test_mode(&self) -> bool {
        self.ppu.get_color_test_mode()
    }

    // APU methods (simplified for cartridge system)
    pub fn initialize_audio(&mut self) {
        // Simplified - no-op for cartridge system
    }

    pub fn is_audio_available(&self) -> bool {
        true // Simplified - assume audio is always available
    }

    pub fn get_audio_info(&self) -> Option<js_sys::Object> {
        let obj = js_sys::Object::new();
        js_sys::Reflect::set(&obj, &"sampleRate".into(), &44100u32.into()).unwrap();
        js_sys::Reflect::set(&obj, &"estimatedLatency".into(), &20u32.into()).unwrap();
        Some(obj)
    }

    pub fn set_master_volume(&mut self, volume: f32) {
        self.apu.set_master_volume(volume);
    }

    pub fn enter_sound_test_mode(&mut self) {
        self.apu.enter_sound_test_mode();
    }

    pub fn exit_sound_test_mode(&mut self) {
        self.apu.exit_sound_test_mode();
    }

    pub fn is_sound_test_mode(&self) -> bool {
        self.apu.is_sound_test_mode()
    }

    pub fn sound_test_change_waveform(&mut self, waveform: u32) {
        self.apu.sound_test_change_waveform(waveform as u8);
    }

    pub fn sound_test_change_note(&mut self, note: u32) {
        self.apu.sound_test_change_note(note as u8);
    }

    pub fn get_current_waveform(&self) -> u32 {
        self.apu.get_current_waveform() as u32
    }

    pub fn get_current_note(&self) -> u32 {
        self.apu.get_current_note() as u32
    }

    pub fn generate_debug_samples(&mut self, count: usize) -> Vec<f32> {
        // Simplified debug sample generation
        let mut samples = Vec::new();
        for _ in 0..count {
            samples.push(self.apu.generate_sample());
        }
        samples
    }

    // Filter controls
    pub fn set_filter_enabled(&mut self, enabled: bool) {
        self.apu.set_filter_enabled(enabled);
    }

    pub fn set_filter_type(&mut self, filter_type: u32) {
        self.apu.set_filter_type(filter_type as u8);
    }

    pub fn set_filter_cutoff(&mut self, cutoff: f32) {
        self.apu.set_filter_cutoff(cutoff);
    }

    pub fn set_filter_resonance(&mut self, resonance: f32) {
        self.apu.set_filter_resonance(resonance);
    }

    // Delay controls
    pub fn set_delay_enabled(&mut self, enabled: bool) {
        self.apu.set_delay_enabled(enabled);
    }

    pub fn set_delay_time(&mut self, delay_time: f32) {
        self.apu.set_delay_time(delay_time);
    }

    pub fn set_delay_feedback(&mut self, feedback: f32) {
        self.apu.set_delay_feedback(feedback);
    }

    pub fn set_delay_mix(&mut self, mix: f32) {
        self.apu.set_delay_mix(mix);
    }

    // Melody controls
    pub fn set_melody_enabled(&mut self, enabled: bool) {
        self.apu.set_melody_enabled(enabled);
    }

    pub fn get_melody_enabled(&self) -> bool {
        self.apu.get_melody_enabled()
    }

    // CPU state for debugging - simplified for cartridge system
    pub fn get_cpu_state(&self) -> js_sys::Object {
        let obj = js_sys::Object::new();
        js_sys::Reflect::set(&obj, &"pc".into(), &0u32.into()).unwrap();
        js_sys::Reflect::set(&obj, &"a".into(), &0u32.into()).unwrap();
        js_sys::Reflect::set(&obj, &"x".into(), &0u32.into()).unwrap();
        js_sys::Reflect::set(&obj, &"y".into(), &0u32.into()).unwrap();
        js_sys::Reflect::set(&obj, &"sp".into(), &0u32.into()).unwrap();
        js_sys::Reflect::set(&obj, &"status".into(), &0u32.into()).unwrap();
        js_sys::Reflect::set(&obj, &"cycles".into(), &0u32.into()).unwrap();
        obj
    }

    pub fn get_frame_count(&self) -> u64 {
        self.ppu.get_frame_count()
    }
}