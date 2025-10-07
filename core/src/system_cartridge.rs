use wasm_bindgen::prelude::*;
use crate::cpu::Cpu;
use crate::ppu_clean::Ppu;
use crate::apu::Apu;
use crate::memory::Memory;
use crate::cartridge::{HambertCartridge, ZSynthCartridge};
use crate::platformer_cartridge::PlatformerCartridge;
use crate::font_system::{FontSystem, Language};
use crate::utils;

#[wasm_bindgen]
pub struct ZebratronCartridgeSystem {
    cpu: Cpu,
    ppu: Ppu,
    apu: Apu,
    memory: Memory,
    hambert_cartridge: Option<HambertCartridge>,
    zsynth_cartridge: Option<ZSynthCartridge>,
    platformer_cartridge: Option<PlatformerCartridge>,
    current_cartridge_type: u8, // 0=none, 1=hambert, 2=zsynth, 3=platformer
    running: bool,
    frame_ready: bool,
    last_game_state: u32, // Track game state changes for audio management
    font_system: FontSystem, // Internationalization support
    debug_animation_frame: u32, // For debug indicator
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
            hambert_cartridge: None,
            zsynth_cartridge: None,
            platformer_cartridge: None,
            current_cartridge_type: 0,
            running: false,
            frame_ready: false,
            last_game_state: 0, // Start with intro state
            font_system: FontSystem::new(), // Initialize font system with English
            debug_animation_frame: 0,
        };
        
        // Set to Japanese language for hiragana text
        system.set_language(1); // 1 = Japanese
        system
    }

    // Load the Hambert cartridge
    pub fn load_hambert_cartridge(&mut self) -> bool {
        let hambert = HambertCartridge::new();
        self.hambert_cartridge = Some(hambert);
        self.zsynth_cartridge = None;
        self.platformer_cartridge = None;
        self.current_cartridge_type = 1;
        self.reset();
        true
    }

    // Load the Z-Synth cartridge
    pub fn load_zsynth_cartridge(&mut self) -> bool {
        let zsynth = ZSynthCartridge::new();
        self.zsynth_cartridge = Some(zsynth);
        self.hambert_cartridge = None;
        self.platformer_cartridge = None;
        self.current_cartridge_type = 2;
        self.reset();
        true
    }

    pub fn load_platformer_cartridge(&mut self) -> bool {
        let platformer = PlatformerCartridge::new();
        self.platformer_cartridge = Some(platformer);
        self.hambert_cartridge = None;
        self.zsynth_cartridge = None;
        self.current_cartridge_type = 3;
        self.reset();
        true
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
        self.running = false;
        self.frame_ready = false;
    }

    pub fn start(&mut self) {
        // Auto-load Platformer cartridge if no cartridge is loaded BEFORE setting running=true
        if self.current_cartridge_type == 0 {
            self.load_platformer_cartridge();
        }
        
        // Set running=true AFTER loading cartridge (so reset() doesn't override it)
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

            // Step APU for sound effect processing
            self.apu.step();

            if frame_complete {
                self.frame_ready = true;

                // Update cartridge game logic every frame
                // Removed noisy debug logging
                match self.current_cartridge_type {
                    1 => {
                        if let Some(ref mut cartridge) = self.hambert_cartridge {
                            cartridge.update_game(false, false, false, false); // No input here, input handled separately
                        }
                    }
                    2 => {
                        if let Some(ref mut cartridge) = self.zsynth_cartridge {
                            cartridge.update_synth(); // Z-Synth doesn't use regular input here
                        }
                    }
                    3 => {
                        // Platformer cartridge handles both frame updates and input in one call
                        // No separate update needed here - handled in input section
                    }
                    _ => {}
                }

                // Sync cartridge data with PPU
                self.sync_cartridge_to_ppu();

                // Process cartridge audio commands
                self.process_cartridge_audio();

                return true;
            }
        }
    }

    // Update cartridge game logic and sync with PPU
    pub fn handle_input(&mut self, up: bool, down: bool, left: bool, right: bool) {
        // Input handling - debug logging removed for performance
        match self.current_cartridge_type {
            1 => {
                if let Some(ref mut cartridge) = self.hambert_cartridge {
                    // Update cartridge with input
                    cartridge.update_game(up, down, left, right);
                }
            }
            2 => {
                // Z-Synth doesn't use directional input
                // Key input is handled separately via handle_zsynth_key methods
            }
            3 => {
                if let Some(ref mut cartridge) = self.platformer_cartridge {
                    // Convert bool inputs to u8 bit flags
                    let mut input_byte = 0u8;
                    if left { input_byte |= 0x01; }
                    if right { input_byte |= 0x02; }
                    if up { input_byte |= 0x04; }
                    if down { input_byte |= 0x08; }
                    // A and B buttons are handled separately
                    cartridge.update(input_byte);
                }
            }
            _ => {}
        }

        // Sync cartridge data with PPU
        self.sync_cartridge_to_ppu();

        // Process cartridge audio commands
        self.process_cartridge_audio();
    }

    fn sync_cartridge_to_ppu(&mut self) {
        match self.current_cartridge_type {
            1 => {
                // Hambert cartridge
                if let Some(ref cartridge) = self.hambert_cartridge {
                    let game_state = cartridge.get_game_state();

                    // Set PPU mode based on game state
                    if game_state == 0 || game_state == 2 { // Intro or Interlude
                        self.ppu.set_intro_mode(true);
                        self.ppu.set_zsynth_mode(false);
                        self.ppu.set_platformer_mode(false);
                        let intro_text = cartridge.get_intro_text_display();
                        self.ppu.set_intro_text(intro_text);
                        // Reset scroll for intro screen
                        self.ppu.set_scroll(0.0, 0.0);
                    } else { // Playing
                        self.ppu.set_intro_mode(false);
                        self.ppu.set_zsynth_mode(false);
                        self.ppu.set_platformer_mode(false);

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
                                let facing_left = js_sys::Reflect::get(&entity_data, &"facing_left".into())
                                    .unwrap()
                                    .as_bool()
                                    .unwrap_or(false);

                                self.ppu.add_sprite(x, y, sprite_id, active, facing_left);
                            }
                        }

                        // Sync player states for visual effects
                        self.ppu.set_lives(cartridge.get_lives());
                        self.ppu.set_player_death_state(cartridge.is_player_dying(), cartridge.get_player_death_flash());
                        self.ppu.set_player_invulnerability_state(cartridge.is_player_invulnerable(), cartridge.get_player_invul_flash());
                    }
                }
            }
            2 => {
                // Z-Synth cartridge - piano keyboard display mode
                self.ppu.set_intro_mode(false);
                self.ppu.set_zsynth_mode(true);
                self.ppu.set_platformer_mode(false);
                
                if let Some(ref cartridge) = self.zsynth_cartridge {
                    // Clear existing sprites
                    self.ppu.clear_sprites();
                    
                    // Add piano keys as visual sprites
                    for i in 0..cartridge.get_piano_key_count() {
                        if let Some(key_data) = cartridge.get_piano_key_data(i) {
                            let x = js_sys::Reflect::get(&key_data, &"x".into())
                                .unwrap()
                                .as_f64()
                                .unwrap_or(0.0) as f32;
                            let y = js_sys::Reflect::get(&key_data, &"y".into())
                                .unwrap()
                                .as_f64()
                                .unwrap_or(0.0) as f32;
                            let is_black = js_sys::Reflect::get(&key_data, &"is_black".into())
                                .unwrap()
                                .as_bool()
                                .unwrap_or(false);
                            let is_pressed = js_sys::Reflect::get(&key_data, &"is_pressed".into())
                                .unwrap()
                                .as_bool()
                                .unwrap_or(false);
                            
                            // Use different sprite IDs for different key states
                            // 10 = white key unpressed, 11 = white key pressed
                            // 12 = black key unpressed, 13 = black key pressed
                            let sprite_id = if is_black {
                                if is_pressed { 13 } else { 12 }
                            } else {
                                if is_pressed { 11 } else { 10 }
                            };
                            
                            self.ppu.add_sprite(x, y, sprite_id, true, false); // Piano keys don't flip
                        }
                    }
                }
                
                self.ppu.set_scroll(0.0, 0.0);
            }
            3 => {
                // Platformer cartridge - simple game display
                self.ppu.set_intro_mode(false);
                self.ppu.set_zsynth_mode(false);
                self.ppu.set_platformer_mode(true);
                
                if let Some(ref cartridge) = self.platformer_cartridge {
                    // Update PPU scroll position
                    let (camera_x, camera_y) = cartridge.get_camera_position();
                    self.ppu.set_scroll(camera_x, camera_y);

                    // Pass level tiles to PPU for rendering
                    let tiles = cartridge.get_tiles();
                    self.ppu.set_platformer_tiles(tiles);

                    // Pass tileset pixel data to PPU (only needed once, but OK to set every frame)
                    let tileset = cartridge.get_tileset();
                    self.ppu.set_platformer_tileset(tileset);

                    // Clear existing sprites
                    self.ppu.clear_sprites();

                    // Add hexagnome sprites
                    let hexagnomes = cartridge.get_hexagnomes();
                    for hexagnome in hexagnomes {
                        let hex_sprite = PlatformerCartridge::get_hexagnome_sprite();
                        let hex_sprite_vec: Vec<Vec<u8>> = hex_sprite.iter().map(|row| row.to_vec()).collect();
                        // Flip horizontally when facing right (hexagnome sprite faces left by default)
                        let flip_horizontal = hexagnome.facing_right;
                        self.ppu.add_sprite_with_data(hexagnome.x, hexagnome.y, &hex_sprite_vec, true, flip_horizontal);
                    }

                    // Add projectile sprites
                    let projectiles = cartridge.get_projectiles();
                    for projectile in projectiles {
                        if projectile.active {
                            let proj_sprite = PlatformerCartridge::get_projectile_sprite();
                            let proj_sprite_vec: Vec<Vec<u8>> = proj_sprite.iter().map(|row| row.to_vec()).collect();
                            self.ppu.add_sprite_with_data(projectile.x, projectile.y, &proj_sprite_vec, true, false);
                        }
                    }

                    // Add player sprite (world position - PPU will apply scroll)
                    let (player_x, player_y) = cartridge.get_player_position();
                    let animation_frame = cartridge.get_animation_frame();
                    let sprite_data = cartridge.get_sprite_data(animation_frame);
                    let flip_horizontal = !cartridge.is_facing_right(); // Flip when facing left
                    let player_sprite_vec: Vec<Vec<u8>> = sprite_data.iter().map(|row| row.to_vec()).collect();
                    self.ppu.add_sprite_with_data(player_x, player_y, &player_sprite_vec, true, flip_horizontal); // Player sprite with animation

                    // Store debug info for after rendering
                    self.debug_animation_frame = animation_frame;

                    // Play background music
                    if cartridge.is_music_enabled() {
                        // Lower the music volume so it doesn't overpower sound effects
                        self.apu.set_sid_volume(0.3); // 30% volume for background music
                        self.play_platformer_music(cartridge.get_music_step());
                    }
                }
            }
            _ => {}
        }
    }

    fn process_cartridge_audio(&mut self) {
        // Check for game state changes and stop audio if transitioning to intro
        if self.current_cartridge_type == 1 {
            if let Some(ref cartridge) = self.hambert_cartridge {
                let current_state = cartridge.get_game_state();
                
                // If we transition to intro (state 0), stop all audio
                if current_state == 0 && self.last_game_state != 0 {
                    self.stop_all_audio();
                }
                
                self.last_game_state = current_state;
            }
        }
        
        match self.current_cartridge_type {
            1 => {
                // Hambert cartridge - process sound effects
                let pending_sounds = if let Some(ref cartridge) = self.hambert_cartridge {
                    cartridge.get_pending_sounds()
                } else {
                    Vec::new()
                };

                // Process each sound effect
                for sound_id in pending_sounds {
                    self.play_sound_effect(sound_id);
                }

                // Clear processed sounds
                if let Some(ref mut cartridge) = self.hambert_cartridge {
                    cartridge.clear_pending_sounds();
                }
            }
            2 => {
                // Z-Synth cartridge - process note on/off events
                if let Some(ref mut cartridge) = self.zsynth_cartridge {
                    let notes_on = cartridge.get_pending_note_on();
                    let notes_off = cartridge.get_pending_note_off();

                    // Process note on events
                    for note in notes_on {
                        self.apu.synth_note_on(note);
                    }

                    // Process note off events
                    for note in notes_off {
                        self.apu.synth_note_off(note);
                    }

                    // Clear processed notes
                    cartridge.clear_pending_notes();
                }
            }
            3 => {
                // Platformer cartridge - process sound effects
                let pending_sounds = if let Some(ref cartridge) = self.platformer_cartridge {
                    cartridge.get_pending_sounds()
                } else {
                    Vec::new()
                };

                // Process each sound effect
                for sound_id in pending_sounds {
                    self.play_platformer_sound_effect(sound_id);
                }

                // Clear processed sounds
                if let Some(ref mut cartridge) = self.platformer_cartridge {
                    cartridge.clear_pending_sounds();
                }
            }
            _ => {}
        }
    }

    fn play_sound_effect(&mut self, sound_id: u32) {
        // Map sound IDs to APU actions
        match sound_id {
            0 => self.play_jump_sound(),      // Jump
            1 => self.play_land_sound(),      // Land
            2 => self.play_collect_sound(),   // Collect
            3 => self.play_enemy_hit_sound(), // Enemy hit
            4 => self.play_shuriken_sound(),  // Shuriken throw
            5 => self.play_death_sound(),     // Death
            6 => self.apu.play_laugh_sample(), // Laughter
            7 => self.apu.play_voice_effect(1), // Gasp
            8 => self.apu.play_voice_effect(2), // Grunt
            _ => {}, // Unknown sound
        }
    }

    fn play_jump_sound(&mut self) {
        // Longer, smoother rising pitch sweep from C4 to G5 over 0.6 seconds
        self.apu.play_sound_effect(60, 79, 1, 0.6); // C4 to G5, sawtooth, 600ms
    }

    fn play_short_jump_sound(&mut self) {
        // Quick, short jump sound - brief upward sweep
        self.apu.play_sound_effect(60, 67, 0, 0.15); // C4 to G4, pulse wave, 150ms
    }

    fn play_land_sound(&mut self) {
        // Short downward thud for landing
        self.apu.play_sound_effect(55, 40, 0, 0.15); // G3 to E2, pulse wave, 150ms
    }

    fn play_collect_sound(&mut self) {
        // Pleasant pickup sound - use timed sound effect
        self.apu.play_sound_effect(72, 84, 3, 0.2); // C5 to C6, sine wave, 200ms
    }

    fn play_enemy_hit_sound(&mut self) {
        // Sharp hit sound - brief noise burst
        self.apu.play_sound_effect(60, 48, 4, 0.1); // C4 to C3, noise, 100ms
    }

    fn play_shuriken_sound(&mut self) {
        // Whoosh sound for projectile - brief triangle wave
        self.apu.play_sound_effect(55, 48, 2, 0.15); // G3 to C3, triangle, 150ms
    }

    fn play_death_sound(&mut self) {
        // Dramatic descending death sound - classic "bonk" effect
        // Start high and sweep down over 1 second for dramatic effect
        self.apu.play_sound_effect(84, 36, 1, 1.0); // C6 down to C2, sawtooth, 1 second
    }

    fn play_platformer_sound_effect(&mut self, sound_id: u32) {
        match sound_id {
            0 => self.play_jump_sound(),
            1 => self.play_land_sound(),
            2 => self.play_short_jump_sound(),
            3 => self.play_collect_sound(),
            4 => self.play_enemy_hit_sound(),
            _ => {} // Unknown sound ID
        }
    }

    pub fn render(&mut self) {
        // Update PPU with current game state
        if let Some(cartridge) = &self.hambert_cartridge {
            self.ppu.set_lives(cartridge.get_lives());
            self.ppu.set_player_death_state(
                cartridge.is_player_dying(),
                cartridge.get_player_death_flash()
            );
        }
        
        self.ppu.render();
        
        // Debug animation frame indicator removed
    }

    pub fn stop_all_audio(&mut self) {
        // Stop all audio when transitioning between game states
        self.apu.exit_sound_test_mode();
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

    // Language switching support for internationalization
    pub fn set_language(&mut self, language: u32) {
        let lang = match language {
            0 => Language::English,
            1 => Language::Japanese,
            _ => Language::English, // Default to English
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

    pub fn generate_debug_samples(&mut self, count: usize) -> Vec<f32> {
        // Simplified debug sample generation
        let mut samples = Vec::new();
        for _ in 0..count {
            samples.push(self.apu.generate_sample());
        }
        samples
    }

    pub fn generate_audio_sample(&mut self) -> f32 {
        self.apu.generate_sample()
    }

    // Get intro text for display (for Japanese hiragana text)
    pub fn get_intro_text(&self) -> String {
        if let Some(ref cartridge) = self.hambert_cartridge {
            cartridge.get_intro_text_display()
        } else {
            String::new()
        }
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

    // Z-Synth specific methods
    pub fn handle_zsynth_key_down(&mut self, key: char) {
        if self.current_cartridge_type == 2 {
            if let Some(ref mut cartridge) = self.zsynth_cartridge {
                cartridge.handle_key_down(key);
                // Process audio immediately for responsive playback
                self.process_cartridge_audio();
            }
        }
    }

    pub fn handle_zsynth_key_up(&mut self, key: char) {
        if self.current_cartridge_type == 2 {
            if let Some(ref mut cartridge) = self.zsynth_cartridge {
                cartridge.handle_key_up(key);
                // Process audio immediately for responsive playback
                self.process_cartridge_audio();
            }
        }
    }

    // Get current cartridge type (0=none, 1=hambert, 2=zsynth)
    pub fn get_current_cartridge_type(&self) -> u8 {
        self.current_cartridge_type
    }

    // MIDI handlers for Z-Synth
    pub fn handle_midi_note_on(&mut self, note: u32) {
        if self.current_cartridge_type == 2 {
            if let Some(ref mut cartridge) = self.zsynth_cartridge {
                cartridge.handle_midi_note_on(note);
                // Process audio immediately for responsive playback
                self.process_cartridge_audio();
            }
        }
    }

    pub fn handle_midi_note_off(&mut self, note: u32) {
        if self.current_cartridge_type == 2 {
            if let Some(ref mut cartridge) = self.zsynth_cartridge {
                cartridge.handle_midi_note_off(note);
                // Process audio immediately for responsive playback
                self.process_cartridge_audio();
            }
        }
    }

    // Get Z-Synth info for display
    pub fn get_zsynth_info(&self) -> String {
        if let Some(ref cartridge) = self.zsynth_cartridge {
            format!("Z-Synth Active - KB Notes: {} | MIDI Notes: {} | APU Notes: {}", 
                cartridge.get_active_note_count(),
                cartridge.get_active_midi_note_count(),
                self.apu.get_synth_active_note_count())
        } else {
            String::from("Z-Synth not loaded")
        }
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

    fn play_platformer_music(&mut self, step: usize) {
        // Simple chip-tune background music for platformer
        // 32-step loop with bass, melody, harmony, and drums

        // Kick drum - very short blip on beats 1 and 3
        if step % 8 == 0 {
            // Very short low frequency blip - 36 (C2) to 24 (C1), pulse wave, 50ms
            self.apu.play_sound_effect(36, 24, 0, 0.05);
        }

        // Snare/hi-hat - short high blip on beats 2 and 4
        if step % 8 == 4 {
            // Very short high frequency blip - 84 (C6) to 72 (C5), noise-like, 40ms
            self.apu.play_sound_effect(84, 72, 0, 0.04);
        }

        // Melody stops - selective stopping for variation
        // Stop on odd steps EXCEPT before phrase endings (7, 15, 23, 31)
        let is_phrase_ending = step % 32 == 6 || step % 32 == 14 || step % 32 == 22 || step % 32 == 30;
        if step % 2 == 1 && !is_phrase_ending {
            self.apu.sid_voice2_stop();
        }

        // Bass stops after 2 steps (keeps bass punchy)
        if step % 4 == 2 {
            self.apu.sid_voice1_stop();
        }

        // Harmony holds longer - only stop right before next harmony note
        if step % 8 == 1 || step % 8 == 5 {
            self.apu.sid_voice3_stop();
        }

        // Bass line (voice 1) - plays on every 4th step
        if step % 4 == 0 {
            let bass_pattern = [36, 36, 43, 0, 41, 41, 38, 0]; // C2, C2, G2, rest, F2, F2, D2, rest
            let bass_note = bass_pattern[(step / 4) % 8];
            if bass_note > 0 {
                self.apu.sid_voice1_play_note(bass_note, 0); // Pulse wave for bass
            } else {
                // Stop on rests
                self.apu.sid_voice1_stop();
            }
        }

        // Melody (voice 2) - plays on every even step
        if step % 2 == 0 {
            let melody_pattern = [
                60, 62, 64, 0,   // C4, D4, E4, rest
                67, 65, 64, 0,   // G4, F4, E4, rest
                60, 64, 67, 72,  // C4, E4, G4, C5 (high note holds)
                67, 0, 60, 0,    // G4, rest, C4, rest
            ];
            let melody_note = melody_pattern[(step / 2) % 16];
            if melody_note > 0 {
                self.apu.sid_voice2_play_note(melody_note, 1); // Sawtooth for melody
            } else {
                // Stop on rests
                self.apu.sid_voice2_stop();
            }
        }

        // Harmony (voice 3) - plays on beats 2 and 4 of each bar (holds longer)
        if step % 8 == 2 || step % 8 == 6 {
            let harmony_pattern = [64, 71, 69, 67]; // E4, B4, A4, G4
            let harmony_note = harmony_pattern[(step / 8) % 4];
            self.apu.sid_voice3_play_note(harmony_note, 2); // Triangle for harmony
        }
    }
}