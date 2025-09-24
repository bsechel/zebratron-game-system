use wasm_bindgen::prelude::*;
use crate::cpu::Cpu;
use crate::ppu_clean::Ppu;
use crate::apu::Apu;
use crate::memory::Memory;
use crate::cartridge::{Cartridge, HambertCartridge, ZSynthCartridge, GameInput, SystemBus, PpuCommand, ApuCommand};
use crate::font_system::{FontSystem, Language};
use crate::utils;

#[wasm_bindgen]
pub struct ZebratronCartridgeSystem {
    cpu: Cpu,
    ppu: Ppu,
    apu: Apu,
    memory: Memory,
    cartridge: Option<Box<dyn Cartridge>>,
    running: bool,
    frame_ready: bool,
    last_game_state: u32, // Track game state changes for audio management
    font_system: FontSystem, // Internationalization support
}

#[wasm_bindgen]
impl ZebratronCartridgeSystem {
    #[wasm_bindgen(constructor)]
    pub fn new() -> ZebratronCartridgeSystem {
        utils::set_panic_hook();

        let mut system = ZebratronCartridgeSystem {
            cpu: Cpu::new(),
            ppu: Ppu::new(),
            apu: Apu::new(),
            memory: Memory::new(),
            cartridge: None,
            running: false,
            frame_ready: false,
            last_game_state: 0, // Start with intro state
            font_system: FontSystem::new(), // Initialize font system with English
        };
        
        system.set_language(1); // 1 = Japanese
        system
    }

    pub fn load_hambert_cartridge(&mut self) -> bool {
        self.cartridge = Some(Box::new(HambertCartridge::new()));
        self.reset();
        true
    }

    pub fn load_zsynth_cartridge(&mut self) -> bool {
        self.cartridge = Some(Box::new(ZSynthCartridge::new()));
        self.reset();
        true
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
        if let Some(cart) = &mut self.cartridge {
            cart.reset();
        }
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
        if !self.running || self.cartridge.is_none() {
            return false;
        }

        loop {
            let frame_complete = self.ppu.step(&self.memory);
            self.apu.step();

            if frame_complete {
                self.frame_ready = true;
                let bus = self.cartridge.as_mut().unwrap().update(&GameInput::default());
                self.process_system_bus(bus);
                return true;
            }
        }
    }

    pub fn handle_input(&mut self, up: bool, down: bool, left: bool, right: bool) {
        if let Some(cart) = &mut self.cartridge {
            let input = GameInput { up, down, left, right, ..Default::default() };
            let bus = cart.update(&input);
            self.process_system_bus(bus);
        }
    }

    fn process_system_bus(&mut self, bus: SystemBus) {
        for cmd in bus.ppu_commands {
            match cmd {
                PpuCommand::SetScroll(x, y) => self.ppu.set_scroll(x, y),
                PpuCommand::AddSprite(sprite) => self.ppu.add_sprite(sprite.x, sprite.y, sprite.sprite_id, sprite.active, sprite.flip_horizontal),
                PpuCommand::ClearSprites => self.ppu.clear_sprites(),
                PpuCommand::SetMode(mode) => {
                    match mode {
                        crate::cartridge::PpuMode::Game => {
                            self.ppu.set_intro_mode(false);
                            self.ppu.set_zsynth_mode(false);
                        }
                        crate::cartridge::PpuMode::Intro => {
                            self.ppu.set_intro_mode(true);
                            self.ppu.set_zsynth_mode(false);
                        }
                        crate::cartridge::PpuMode::ZSynth => {
                            self.ppu.set_intro_mode(false);
                            self.ppu.set_zsynth_mode(true);
                        }
                    }
                }
                PpuCommand::SetHudData(hud) => {
                    self.ppu.set_lives(hud.lives);
                    self.ppu.set_player_death_state(hud.is_dying, hud.death_flash);
                    self.ppu.set_player_invulnerability_state(hud.is_invulnerable, hud.invuln_flash);
                    if let Some(text) = hud.intro_text {
                        self.ppu.set_intro_text(text);
                    }
                }
            }
        }

        for cmd in bus.apu_commands {
            match cmd {
                ApuCommand::PlaySoundEffect(id) => self.play_sound_effect(id),
                ApuCommand::SynthNoteOn(note) => self.apu.synth_note_on(note),
                ApuCommand::SynthNoteOff(note) => self.apu.synth_note_off(note),
                ApuCommand::StopAllAudio => self.stop_all_audio(),
            }
        }
    }

    fn play_sound_effect(&mut self, sound_id: u32) {
        match sound_id {
            0 => self.apu.play_sound_effect(60, 79, 1, 0.6),      // Jump
            1 => self.apu.play_sound_effect(55, 40, 0, 0.15),     // Land
            2 => self.apu.play_sound_effect(72, 84, 3, 0.2),      // Collect
            3 => self.apu.play_sound_effect(60, 48, 4, 0.1),      // Enemy hit
            4 => self.apu.play_sound_effect(55, 48, 2, 0.15),     // Shuriken throw
            5 => self.apu.play_sound_effect(84, 36, 1, 1.0),      // Death
            6 => self.apu.play_laugh_sample(),                     // Laughter
            7 => self.apu.play_voice_effect(1),                    // Gasp
            8 => self.apu.play_voice_effect(2),                    // Grunt
            _ => {},
        }
    }

    pub fn render(&mut self) {
        self.ppu.render();
    }

    pub fn stop_all_audio(&mut self) {
        self.apu.exit_sound_test_mode();
        self.apu.sid_stop_all();
        self.apu.poly_stop_all();
    }

    pub fn get_screen_buffer(&self) -> js_sys::Uint8Array {
        let buffer = self.ppu.get_screen_buffer();
        js_sys::Uint8Array::from(&buffer[..])
    }

    pub fn toggle_color_test(&mut self) {
        self.ppu.toggle_color_test();
    }

    pub fn get_color_test_mode(&self) -> bool {
        self.ppu.get_color_test_mode()
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

    pub fn set_language(&mut self, language: u32) {
        let lang = match language {
            0 => Language::English,
            1 => Language::Japanese,
            _ => Language::English,
        };
        self.font_system.set_language(lang);
        self.ppu.set_language(lang);
    }

    pub fn get_language(&self) -> u32 {
        match self.font_system.current_language {
            Language::English => 0,
            Language::Japanese => 1,
        }
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

    pub fn generate_audio_sample(&mut self) -> f32 {
        self.apu.generate_sample()
    }

    pub fn get_intro_text(&self) -> String {
        // This might need to be handled differently now
        // For now, we can't easily get text back from the cartridge
        String::new()
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

    pub fn get_frame_count(&self) -> u64 {
        self.ppu.get_frame_count()
    }

    // Z-Synth and other direct input handlers
    pub fn handle_zsynth_key_down(&mut self, key: char) {
        if let Some(cart) = &mut self.cartridge {
            cart.handle_key_down(key);
        }
    }

    pub fn handle_zsynth_key_up(&mut self, key: char) {
        if let Some(cart) = &mut self.cartridge {
            cart.handle_key_up(key);
        }
    }

    pub fn handle_midi_note_on(&mut self, note: u32) {
        if let Some(cart) = &mut self.cartridge {
            cart.handle_midi_note_on(note);
        }
    }

    pub fn handle_midi_note_off(&mut self, note: u32) {
        if let Some(cart) = &mut self.cartridge {
            cart.handle_midi_note_off(note);
        }
    }

    pub fn get_zsynth_info(&self) -> String {
        // This would need a method on the trait to get info, if desired
        String::from("Info not available from generic cartridge")
    }

    // SID-style 3-voice API delegation to APU
    pub fn sid_voice1_play_note(&mut self, note: u8, waveform: u8) {
        self.apu.sid_voice1_play_note(note, waveform);
    }

    pub fn sid_voice2_play_note(&mut self, note: u8, waveform: u8) {
        self.apu.sid_voice2_play_note(note, waveform);
    }

    pub fn sid_voice3_play_note(&mut self, note: u8, waveform: u8) {
        self.apu.sid_voice3_play_note(note, waveform);
    }

    pub fn sid_voice1_stop(&mut self) {
        self.apu.sid_voice1_stop();
    }

    pub fn sid_voice2_stop(&mut self) {
        self.apu.sid_voice2_stop();
    }

    pub fn sid_voice3_stop(&mut self) {
        self.apu.sid_voice3_stop();
    }

    pub fn sid_stop_all(&mut self) {
        self.apu.sid_stop_all();
    }

    pub fn set_sid_volume(&mut self, volume: f32) {
        self.apu.set_sid_volume(volume);
    }

    pub fn set_poly_volume(&mut self, volume: f32) {
        self.apu.set_poly_volume(volume);
    }

    pub fn sid_set_filter_voices(&mut self, voice1: bool, voice2: bool, voice3: bool) {
        self.apu.sid_set_filter_voices(voice1, voice2, voice3);
    }

    pub fn sid_set_filter_cutoff(&mut self, cutoff: f32) {
        self.apu.sid_set_filter_cutoff(cutoff);
    }

    pub fn sid_set_filter_resonance(&mut self, resonance: f32) {
        self.apu.sid_set_filter_resonance(resonance);
    }

    pub fn sid_set_filter_type(&mut self, filter_type: u8) {
        self.apu.sid_set_filter_type(filter_type);
    }

    // Polyphonic layer API delegation to APU
    pub fn poly_play_chord(&mut self, notes: &[u8]) {
        self.apu.poly_play_chord(notes.to_vec());
    }

    pub fn poly_play_note(&mut self, note: u8) {
        self.apu.poly_play_note(note);
    }

    pub fn poly_stop_note(&mut self, note: u8) {
        self.apu.poly_stop_note(note);
    }

    pub fn poly_stop_all(&mut self) {
        self.apu.poly_stop_all();
    }
}