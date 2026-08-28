#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;
use crate::cpu::Cpu;
use crate::ppu_clean::Ppu;
use crate::apu::Apu;
use crate::memory::Memory;
use crate::cartridge::{HambertCartridge, ZSynthCartridge};
use crate::platformer_cartridge::PlatformerCartridge;
use crate::font_system::{FontSystem, Language};
use crate::utils;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
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
    last_text_index: usize, // Cache text index to avoid String allocations every frame
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl ZebratronCartridgeSystem {
    #[cfg_attr(feature = "wasm", wasm_bindgen(constructor))]
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
            last_text_index: 0,
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

    pub fn skip_to_gameplay(&mut self) {
        if let Some(ref mut platformer) = self.platformer_cartridge {
            platformer.skip_to_gameplay();
        }
        // Clear PPU cutscene/title state
        self.ppu.set_cutscene_mode(false);
        self.ppu.set_platformer_mode(true);
        self.ppu.set_title_screen_mode(false);
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
        // PPU runs at 3x CPU speed, so step APU every 3 PPU cycles
        let mut ppu_cycle = 0;
        loop {
            let frame_complete = self.ppu.step(&self.memory);

            // Step APU at CPU speed (every 3 PPU cycles for authentic NES timing)
            // This reduces APU calls from 89,342 to 29,780 per frame
            ppu_cycle += 1;
            if ppu_cycle % 3 == 0 {
                self.apu.step();
            }

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
    pub fn handle_input(&mut self, up: bool, down: bool, left: bool, right: bool, a_button: bool, b_button: bool) {
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
                    if a_button { input_byte |= 0x10; } // A button (bit 4)
                    if b_button { input_byte |= 0x20; } // B button (bit 5)
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

                        // MEMORY FIX: Only update intro text when it changes (avoid 60 String allocations/second)
                        let current_text_index = cartridge.get_text_index();
                        if current_text_index != self.last_text_index {
                            let intro_text = cartridge.get_intro_text_display();
                            self.ppu.set_intro_text(intro_text);
                            self.last_text_index = current_text_index;
                        }

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

                        // ZERO-ALLOCATION ENTITY SYNC: Use direct field access instead of struct allocation
                        // This eliminates memory allocations that were causing crashes
                        for i in 0..cartridge.get_entity_count() {
                            let x = cartridge.get_entity_x(i);
                            let y = cartridge.get_entity_y(i);
                            let sprite_id = cartridge.get_entity_sprite_id(i);
                            let active = cartridge.get_entity_active(i);
                            let facing_left = cartridge.get_entity_facing_left(i);

                            self.ppu.add_sprite(x, y, sprite_id, active, facing_left);
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
                        if let Some(key_data) = cartridge.get_piano_key_data_native(i) {
                            let x = key_data.x;
                            let y = key_data.y;
                            let is_black = key_data.is_black;
                            let is_pressed = key_data.is_pressed;
                            
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
                // Platformer cartridge - check game state
                if let Some(ref cartridge) = self.platformer_cartridge {
                    let game_state = cartridge.get_game_state();
                    let is_title_screen = matches!(game_state, crate::platformer_cartridge::GameState::TitleScreen);
                    let is_cutscene = matches!(game_state, crate::platformer_cartridge::GameState::Cutscene);

                    if is_cutscene {
                        // Cutscene mode
                        self.ppu.set_intro_mode(false);
                        self.ppu.set_zsynth_mode(false);
                        self.ppu.set_platformer_mode(false);
                        self.ppu.set_title_screen_mode(false);
                        self.ppu.set_cutscene_mode(true);

                        // Pass cutscene data to PPU
                        if let Some(cutscene) = cartridge.get_current_cutscene() {
                            if cutscene.current_screen < cutscene.screens.len() {
                                let screen = &cutscene.screens[cutscene.current_screen];

                                // Pass image
                                let image_vec: Vec<Vec<u8>> = screen.image.iter().map(|row| row.to_vec()).collect();
                                self.ppu.set_cutscene_image(image_vec);

                                // Pass text
                                self.ppu.set_cutscene_text(screen.text_lines.clone());

                                // Pass scroll offset
                                self.ppu.set_cutscene_scroll_offset(cutscene.text_scroll_offset);

                                // Pass character index for typing effect
                                self.ppu.set_cutscene_char_index(cutscene.text_char_index);
                            }
                        }
                    } else if is_title_screen {
                        // Title screen mode
                        self.ppu.set_intro_mode(false);
                        self.ppu.set_zsynth_mode(false);
                        self.ppu.set_platformer_mode(false);
                        self.ppu.set_title_screen_mode(true);

                        // Pass title logo data to PPU
                        let logo = cartridge.get_title_logo();
                        let logo_vec: Vec<Vec<u8>> = logo.iter().map(|row| row.to_vec()).collect();
                        self.ppu.set_title_logo(logo_vec);

                        // Set press start visibility
                        self.ppu.set_show_press_start(cartridge.should_show_press_start());
                    } else {
                        // Gameplay mode
                        self.ppu.set_intro_mode(false);
                        self.ppu.set_zsynth_mode(false);
                        self.ppu.set_platformer_mode(true);
                        self.ppu.set_title_screen_mode(false);
                        self.ppu.set_cutscene_mode(false);

                        // Update lives display
                        self.ppu.set_lives(cartridge.get_lives());

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

                        // Update HUD state (lives and invulnerability)
                        let lives = cartridge.get_lives();
                        self.ppu.set_lives(lives);

                        let is_invulnerable = cartridge.is_invulnerable();
                        let should_flash = is_invulnerable && ((self.ppu.get_frame_count() / 4) % 2 == 0); // Flash every 4 frames
                        self.ppu.set_player_invulnerability_state(is_invulnerable, should_flash);

                        // Get all sprites from cartridge and render them
                        let sprites = cartridge.get_all_sprites();
                        for (idx, sprite) in sprites.iter().enumerate() {
                            // Skip player sprite if invulnerable and flashing
                            if idx == 0 && should_flash {
                                continue; // First sprite is always the player
                            }
                            self.ppu.add_sprite_with_data(sprite.x, sprite.y, &sprite.sprite_data, true, sprite.flip_horizontal, sprite.scale);
                        }

                        // Store debug info for after rendering
                        self.debug_animation_frame = cartridge.get_animation_frame();

                        // Play background music (only if not dying)
                        if cartridge.is_music_enabled() && !cartridge.is_dying() {
                            // Lower the music volume so it doesn't overpower sound effects
                            self.apu.set_sid_volume(0.2); // 20% volume for background music (lowered from 30%)
                            self.apu.set_sid_voice2_volume(0.2); // Lead melody at 20% of voice volume
                            self.apu.set_sid_voice3_volume(0.2); // Upper harmony at 20% of voice volume
                            self.play_platformer_midi_music(
                                cartridge.get_current_lead_note(),
                                cartridge.get_current_upper_note(),
                                cartridge.get_current_bass_note(),
                                cartridge.should_play_kick(),
                                cartridge.should_play_snare(),
                                cartridge.should_play_lead(),
                                cartridge.should_stop_lead(),
                                cartridge.should_play_upper(),
                                cartridge.should_stop_upper(),
                                cartridge.should_play_bass(),
                                cartridge.should_stop_bass()
                            );
                        } else if cartridge.is_dying() {
                            // Stop all music voices when dying
                            self.apu.sid_voice1_stop();
                            self.apu.sid_voice2_stop();
                            self.apu.sid_voice3_stop();
                        }
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
                // MEMORY FIX: Only get sounds if they exist (avoid 60 Vec allocations/second)
                let pending_sounds = if let Some(ref cartridge) = self.hambert_cartridge {
                    if cartridge.has_pending_sounds() {
                        cartridge.get_pending_sounds()
                    } else {
                        Vec::new()
                    }
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
                // MEMORY FIX: Only get sounds if they exist (avoid 60 Vec allocations/second)
                let pending_sounds = if let Some(ref cartridge) = self.platformer_cartridge {
                    if cartridge.has_pending_sounds() {
                        cartridge.get_pending_sounds()
                    } else {
                        Vec::new()
                    }
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
            10 => self.play_punch_sound(),    // Punch attack
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
        // Pleasant pickup sound - short upward chime/plink
        self.apu.play_sound_effect(84, 96, 3, 0.1); // C6 to C7, sine wave, 100ms - quick pleasant chime
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

    fn play_punch_sound(&mut self) {
        // Powerful punch sound - bass sweep for impact
        self.apu.play_sound_effect(72, 24, 0, 0.12); // C5 to C1, pulse wave, 120ms - sharp descending punch
    }

    fn play_platformer_sound_effect(&mut self, sound_id: u32) {
        match sound_id {
            0 => self.play_jump_sound(),
            1 => self.play_land_sound(),
            2 => self.play_short_jump_sound(),
            3 => self.play_collect_sound(),
            4 => self.play_enemy_hit_sound(),
            7 => self.play_text_blip_sound(),  // Text typing blip (like NES RPGs)
            8 => self.play_death_sound(),      // Death sound (Hambert flies off screen)
            10 => self.play_punch_sound(),     // Punch attack
            _ => {} // Unknown sound ID
        }
    }

    fn play_text_blip_sound(&mut self) {
        // Very short noise burst for text typing - subtle click sound
        self.apu.play_sound_effect(72, 72, 4, 0.05); // C5, noise waveform, 50ms
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

        if let Some(cartridge) = &self.platformer_cartridge {
            self.ppu.set_lives(cartridge.get_lives());
            self.ppu.set_hamberries(cartridge.get_hamberries_collected());
        }

        self.ppu.render();
        
        // Debug animation frame indicator removed
    }

    pub fn stop_all_audio(&mut self) {
        // Stop all audio when transitioning between game states
        self.apu.exit_sound_test_mode();
    }

    #[cfg(feature = "wasm")]
    pub fn get_screen_buffer(&self) -> js_sys::Uint8Array {
        let buffer = self.ppu.get_screen_buffer_ref();
        unsafe { js_sys::Uint8Array::view(buffer) }
    }

    #[cfg(not(feature = "wasm"))]
    pub fn get_screen_buffer_vec(&self) -> Vec<u8> {
        self.ppu.get_screen_buffer_vec()
    }

    // PPU control methods
    pub fn toggle_color_test(&mut self) {
        self.ppu.toggle_color_test();
    }

    pub fn get_color_test_mode(&self) -> bool {
        self.ppu.get_color_test_mode()
    }

    // Palette cycling methods for animated effects
    pub fn cycle_palette_range(&mut self, start: usize, end: usize) {
        self.ppu.cycle_palette_range(start, end);
    }

    pub fn reset_palette(&mut self) {
        self.ppu.reset_palette();
    }

    // APU methods (simplified for cartridge system)
    pub fn initialize_audio(&mut self) {
        // Simplified - no-op for cartridge system
    }

    pub fn is_audio_available(&self) -> bool {
        true // Simplified - assume audio is always available
    }

    #[cfg(feature = "wasm")]
    pub fn get_audio_info(&self) -> Option<js_sys::Object> {
        let obj = js_sys::Object::new();
        js_sys::Reflect::set(&obj, &"sampleRate".into(), &44100u32.into()).unwrap();
        js_sys::Reflect::set(&obj, &"estimatedLatency".into(), &20u32.into()).unwrap();
        Some(obj)
    }

    #[cfg(not(feature = "wasm"))]
    pub fn get_audio_info_json(&self) -> String {
        serde_json::json!({
            "sampleRate": 44100,
            "estimatedLatency": 20
        }).to_string()
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
    #[cfg(feature = "wasm")]
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

    #[cfg(not(feature = "wasm"))]
    pub fn get_cpu_state_json(&self) -> String {
        serde_json::json!({
            "pc": 0,
            "a": 0,
            "x": 0,
            "y": 0,
            "sp": 0,
            "status": 0,
            "cycles": 0
        }).to_string()
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

    fn play_platformer_midi_music(&mut self, lead_note: Option<u32>, upper_note: Option<u32>, bass_note: Option<u32>, should_play_kick: bool, should_play_snare: bool, should_play_lead: bool, should_stop_lead: bool, should_play_upper: bool, should_stop_upper: bool, should_play_bass: bool, should_stop_bass: bool) {
        // Play MIDI-imported music for Level 1
        // Voice 1: Bass
        // Voice 2: Lead melody (or lower harmony in pattern 2)
        // Voice 3: Upper harmony (pattern 2 only)

        // Play bass on voice 1 (pulse wave) with independent timing
        if should_play_bass {
            if let Some(note) = bass_note {
                self.apu.sid_voice1_play_note(note as u8, 0);
            }
        }

        // Stop bass after note duration
        if should_stop_bass {
            self.apu.sid_voice1_stop();
        }

        // Play lead melody on voice 2 (pulse wave) - respect note durations.
        // note == 0 is a rest sentinel: should_play_lead still fires on schedule,
        // but we skip triggering so the voice stays silent for that entry's duration.
        if should_play_lead {
            if let Some(note) = lead_note {
                if note != 0 {
                    self.apu.sid_voice2_play_note(note as u8, 0);
                }
            }
        }

        // Stop lead melody after note duration
        if should_stop_lead {
            self.apu.sid_voice2_stop();
        }

        // Upper harmony (voice 3, pattern 2) temporarily silenced while dialing in
        // the new lead melody. To restore: uncomment below.
        let _ = (should_play_upper, upper_note, should_stop_upper);
        // if should_play_upper {
        //     if let Some(note) = upper_note {
        //         self.apu.sid_voice3_play_note(note as u8, 0);
        //     }
        // }
        // if should_stop_upper {
        //     self.apu.sid_voice3_stop();
        // }

        // Kick drum, four-on-the-floor
        if should_play_kick {
            self.apu.trigger_percussion(0, 0.35);
        }

        // Snare on third beat of bar
        if should_play_snare {
            self.apu.trigger_percussion(1, 0.5);
        }
    }
}