use wasm_bindgen::prelude::*;
use crate::memory::Memory;

const SCREEN_WIDTH: usize = 320;
const SCREEN_HEIGHT: usize = 240;

// 8x8 bitmap font data for printable ASCII characters (32-126)
const FONT_8X8: [[u8; 8]; 95] = [
    // Space (32)
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    // ! (33)
    [0x18, 0x3C, 0x3C, 0x18, 0x18, 0x00, 0x18, 0x00],
    // " (34)
    [0x36, 0x36, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    // # (35)
    [0x36, 0x36, 0x7F, 0x36, 0x7F, 0x36, 0x36, 0x00],
    // $ (36)
    [0x0C, 0x3E, 0x03, 0x1E, 0x30, 0x1F, 0x0C, 0x00],
    // % (37)
    [0x00, 0x63, 0x33, 0x18, 0x0C, 0x66, 0x63, 0x00],
    // & (38)
    [0x1C, 0x36, 0x1C, 0x6E, 0x3B, 0x33, 0x6E, 0x00],
    // ' (39)
    [0x06, 0x06, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00],
    // ( (40)
    [0x18, 0x0C, 0x06, 0x06, 0x06, 0x0C, 0x18, 0x00],
    // ) (41)
    [0x06, 0x0C, 0x18, 0x18, 0x18, 0x0C, 0x06, 0x00],
    // * (42)
    [0x00, 0x66, 0x3C, 0xFF, 0x3C, 0x66, 0x00, 0x00],
    // + (43)
    [0x00, 0x0C, 0x0C, 0x3F, 0x0C, 0x0C, 0x00, 0x00],
    // , (44)
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x0C, 0x06, 0x00],
    // - (45)
    [0x00, 0x00, 0x00, 0x3F, 0x00, 0x00, 0x00, 0x00],
    // . (46)
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x0C, 0x0C, 0x00],
    // / (47)
    [0x60, 0x30, 0x18, 0x0C, 0x06, 0x03, 0x01, 0x00],
    // 0 (48)
    [0x3E, 0x63, 0x73, 0x7B, 0x6F, 0x67, 0x3E, 0x00],
    // 1 (49)
    [0x0C, 0x0E, 0x0C, 0x0C, 0x0C, 0x0C, 0x3F, 0x00],
    // 2 (50)
    [0x1E, 0x33, 0x30, 0x1C, 0x06, 0x33, 0x3F, 0x00],
    // 3 (51)
    [0x1E, 0x33, 0x30, 0x1C, 0x30, 0x33, 0x1E, 0x00],
    // 4 (52)
    [0x38, 0x3C, 0x36, 0x33, 0x7F, 0x30, 0x78, 0x00],
    // 5 (53)
    [0x3F, 0x03, 0x1F, 0x30, 0x30, 0x33, 0x1E, 0x00],
    // 6 (54)
    [0x1C, 0x06, 0x03, 0x1F, 0x33, 0x33, 0x1E, 0x00],
    // 7 (55)
    [0x3F, 0x33, 0x30, 0x18, 0x0C, 0x0C, 0x0C, 0x00],
    // 8 (56)
    [0x1E, 0x33, 0x33, 0x1E, 0x33, 0x33, 0x1E, 0x00],
    // 9 (57)
    [0x1E, 0x33, 0x33, 0x3E, 0x30, 0x18, 0x0E, 0x00],
    // : (58)
    [0x00, 0x0C, 0x0C, 0x00, 0x00, 0x0C, 0x0C, 0x00],
    // ; (59)
    [0x00, 0x0C, 0x0C, 0x00, 0x00, 0x0C, 0x06, 0x00],
    // < (60)
    [0x18, 0x0C, 0x06, 0x03, 0x06, 0x0C, 0x18, 0x00],
    // = (61)
    [0x00, 0x00, 0x3F, 0x00, 0x00, 0x3F, 0x00, 0x00],
    // > (62)
    [0x06, 0x0C, 0x18, 0x30, 0x18, 0x0C, 0x06, 0x00],
    // ? (63)
    [0x1E, 0x33, 0x30, 0x18, 0x0C, 0x00, 0x0C, 0x00],
    // @ (64)
    [0x3E, 0x63, 0x7B, 0x7B, 0x7B, 0x03, 0x1E, 0x00],
    // A (65)
    [0x0C, 0x1E, 0x33, 0x33, 0x3F, 0x33, 0x33, 0x00],
    // B (66)
    [0x3F, 0x66, 0x66, 0x3E, 0x66, 0x66, 0x3F, 0x00],
    // C (67)
    [0x3C, 0x66, 0x03, 0x03, 0x03, 0x66, 0x3C, 0x00],
    // D (68)
    [0x1F, 0x36, 0x66, 0x66, 0x66, 0x36, 0x1F, 0x00],
    // E (69)
    [0x7F, 0x46, 0x16, 0x1E, 0x16, 0x46, 0x7F, 0x00],
    // F (70)
    [0x7F, 0x46, 0x16, 0x1E, 0x16, 0x06, 0x0F, 0x00],
    // G (71)
    [0x3C, 0x66, 0x03, 0x03, 0x73, 0x66, 0x7C, 0x00],
    // H (72)
    [0x33, 0x33, 0x33, 0x3F, 0x33, 0x33, 0x33, 0x00],
    // I (73)
    [0x1E, 0x0C, 0x0C, 0x0C, 0x0C, 0x0C, 0x1E, 0x00],
    // J (74)
    [0x78, 0x30, 0x30, 0x30, 0x33, 0x33, 0x1E, 0x00],
    // K (75)
    [0x67, 0x66, 0x36, 0x1E, 0x36, 0x66, 0x67, 0x00],
    // L (76)
    [0x0F, 0x06, 0x06, 0x06, 0x46, 0x66, 0x7F, 0x00],
    // M (77)
    [0x63, 0x77, 0x7F, 0x7F, 0x6B, 0x63, 0x63, 0x00],
    // N (78)
    [0x63, 0x67, 0x6F, 0x7B, 0x73, 0x63, 0x63, 0x00],
    // O (79)
    [0x1C, 0x36, 0x63, 0x63, 0x63, 0x36, 0x1C, 0x00],
    // P (80)
    [0x3F, 0x66, 0x66, 0x3E, 0x06, 0x06, 0x0F, 0x00],
    // Q (81)
    [0x1E, 0x33, 0x33, 0x33, 0x3B, 0x1E, 0x38, 0x00],
    // R (82)
    [0x3F, 0x66, 0x66, 0x3E, 0x36, 0x66, 0x67, 0x00],
    // S (83)
    [0x1E, 0x33, 0x07, 0x0E, 0x38, 0x33, 0x1E, 0x00],
    // T (84)
    [0x3F, 0x2D, 0x0C, 0x0C, 0x0C, 0x0C, 0x1E, 0x00],
    // U (85)
    [0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x3F, 0x00],
    // V (86)
    [0x33, 0x33, 0x33, 0x33, 0x33, 0x1E, 0x0C, 0x00],
    // W (87)
    [0x63, 0x63, 0x63, 0x6B, 0x7F, 0x77, 0x63, 0x00],
    // X (88)
    [0x63, 0x63, 0x36, 0x1C, 0x1C, 0x36, 0x63, 0x00],
    // Y (89)
    [0x33, 0x33, 0x33, 0x1E, 0x0C, 0x0C, 0x1E, 0x00],
    // Z (90)
    [0x7F, 0x63, 0x31, 0x18, 0x4C, 0x66, 0x7F, 0x00],
    // [ (91)
    [0x1E, 0x06, 0x06, 0x06, 0x06, 0x06, 0x1E, 0x00],
    // \ (92)
    [0x03, 0x06, 0x0C, 0x18, 0x30, 0x60, 0x40, 0x00],
    // ] (93)
    [0x1E, 0x18, 0x18, 0x18, 0x18, 0x18, 0x1E, 0x00],
    // ^ (94)
    [0x08, 0x1C, 0x36, 0x63, 0x00, 0x00, 0x00, 0x00],
    // _ (95)
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF],
    // ` (96)
    [0x0C, 0x0C, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00],
    // a (97)
    [0x00, 0x00, 0x1E, 0x30, 0x3E, 0x33, 0x6E, 0x00],
    // b (98)
    [0x07, 0x06, 0x06, 0x3E, 0x66, 0x66, 0x3B, 0x00],
    // c (99)
    [0x00, 0x00, 0x1E, 0x33, 0x03, 0x33, 0x1E, 0x00],
    // d (100)
    [0x38, 0x30, 0x30, 0x3e, 0x33, 0x33, 0x6E, 0x00],
    // e (101)
    [0x00, 0x00, 0x1E, 0x33, 0x3f, 0x03, 0x1E, 0x00],
    // f (102)
    [0x1C, 0x36, 0x06, 0x0f, 0x06, 0x06, 0x0F, 0x00],
    // g (103)
    [0x00, 0x00, 0x6E, 0x33, 0x33, 0x3E, 0x30, 0x1F],
    // h (104)
    [0x07, 0x06, 0x36, 0x6E, 0x66, 0x66, 0x67, 0x00],
    // i (105)
    [0x0C, 0x00, 0x0E, 0x0C, 0x0C, 0x0C, 0x1E, 0x00],
    // j (106)
    [0x30, 0x00, 0x30, 0x30, 0x30, 0x33, 0x33, 0x1E],
    // k (107)
    [0x07, 0x06, 0x66, 0x36, 0x1E, 0x36, 0x67, 0x00],
    // l (108)
    [0x0E, 0x0C, 0x0C, 0x0C, 0x0C, 0x0C, 0x1E, 0x00],
    // m (109)
    [0x00, 0x00, 0x33, 0x7F, 0x7F, 0x6B, 0x63, 0x00],
    // n (110)
    [0x00, 0x00, 0x1F, 0x33, 0x33, 0x33, 0x33, 0x00],
    // o (111)
    [0x00, 0x00, 0x1E, 0x33, 0x33, 0x33, 0x1E, 0x00],
    // p (112)
    [0x00, 0x00, 0x3B, 0x66, 0x66, 0x3E, 0x06, 0x0F],
    // q (113)
    [0x00, 0x00, 0x6E, 0x33, 0x33, 0x3E, 0x30, 0x78],
    // r (114)
    [0x00, 0x00, 0x3B, 0x6E, 0x66, 0x06, 0x0F, 0x00],
    // s (115)
    [0x00, 0x00, 0x3E, 0x03, 0x1E, 0x30, 0x1F, 0x00],
    // t (116)
    [0x08, 0x0C, 0x3E, 0x0C, 0x0C, 0x2C, 0x18, 0x00],
    // u (117)
    [0x00, 0x00, 0x33, 0x33, 0x33, 0x33, 0x6E, 0x00],
    // v (118)
    [0x00, 0x00, 0x33, 0x33, 0x33, 0x1E, 0x0C, 0x00],
    // w (119)
    [0x00, 0x00, 0x63, 0x6B, 0x7F, 0x7F, 0x36, 0x00],
    // x (120)
    [0x00, 0x00, 0x63, 0x36, 0x1C, 0x36, 0x63, 0x00],
    // y (121)
    [0x00, 0x00, 0x33, 0x33, 0x33, 0x3E, 0x30, 0x1F],
    // z (122)
    [0x00, 0x00, 0x3F, 0x19, 0x0C, 0x26, 0x3F, 0x00],
    // { (123)
    [0x38, 0x0C, 0x0C, 0x07, 0x0C, 0x0C, 0x38, 0x00],
    // | (124)
    [0x18, 0x18, 0x18, 0x00, 0x18, 0x18, 0x18, 0x00],
    // } (125)
    [0x07, 0x0C, 0x0C, 0x38, 0x0C, 0x0C, 0x07, 0x00],
    // ~ (126)
    [0x6E, 0x3B, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
];

// 128-color master palette - artist-friendly with good range
const MASTER_PALETTE: [(u8, u8, u8); 128] = [
    // Grayscale ramp (0-15)
    (0, 0, 0), (17, 17, 17), (34, 34, 34), (51, 51, 51),
    (68, 68, 68), (85, 85, 85), (102, 102, 102), (119, 119, 119),
    (136, 136, 136), (153, 153, 153), (170, 170, 170), (187, 187, 187),
    (204, 204, 204), (221, 221, 221), (238, 238, 238), (255, 255, 255),

    // Reds (16-31)
    (128, 0, 0), (160, 0, 0), (192, 0, 0), (224, 0, 0),
    (255, 0, 0), (255, 32, 32), (255, 64, 64), (255, 96, 96),
    (255, 128, 128), (255, 160, 160), (255, 192, 192), (255, 224, 224),
    (128, 32, 0), (160, 64, 0), (192, 96, 32), (224, 128, 64),

    // Oranges/Browns (32-47)
    (255, 128, 0), (255, 160, 0), (255, 192, 0), (255, 224, 0),
    (255, 255, 0), (224, 224, 0), (192, 192, 0), (160, 160, 0),
    (128, 128, 0), (160, 128, 64), (192, 160, 96), (224, 192, 128),
    (139, 69, 19), (160, 82, 45), (205, 133, 63), (222, 184, 135),

    // Greens (48-63)
    (0, 128, 0), (0, 160, 0), (0, 192, 0), (0, 224, 0),
    (0, 255, 0), (32, 255, 32), (64, 255, 64), (96, 255, 96),
    (128, 255, 128), (160, 255, 160), (192, 255, 192), (224, 255, 224),
    (0, 128, 64), (0, 160, 96), (32, 192, 128), (64, 224, 160),

    // Cyans (64-79)
    (0, 255, 255), (0, 224, 224), (0, 192, 192), (0, 160, 160),
    (0, 128, 128), (32, 160, 160), (64, 192, 192), (96, 224, 224),
    (128, 255, 255), (160, 255, 255), (192, 255, 255), (224, 255, 255),
    (0, 128, 96), (0, 160, 128), (32, 192, 160), (64, 224, 192),

    // Blues (80-95)
    (0, 0, 128), (0, 0, 160), (0, 0, 192), (0, 0, 224),
    (0, 0, 255), (32, 32, 255), (64, 64, 255), (96, 96, 255),
    (128, 128, 255), (160, 160, 255), (192, 192, 255), (224, 224, 255),
    (0, 64, 128), (32, 96, 160), (64, 128, 192), (96, 160, 224),

    // Purples/Magentas (96-111)
    (128, 0, 128), (160, 0, 160), (192, 0, 192), (224, 0, 224),
    (255, 0, 255), (255, 32, 255), (255, 64, 255), (255, 96, 255),
    (255, 128, 255), (255, 160, 255), (255, 192, 255), (255, 224, 255),
    (128, 0, 64), (160, 32, 96), (192, 64, 128), (224, 96, 160),

    // Skin tones & earth tones (112-127)
    (255, 220, 177), (255, 206, 158), (238, 180, 120), (210, 150, 95),
    (180, 120, 80), (150, 100, 70), (120, 80, 60), (100, 70, 50),
    (139, 115, 85), (160, 130, 98), (205, 175, 149), (222, 196, 176),
    (245, 222, 179), (255, 228, 196), (255, 235, 205), (255, 248, 220),
];

// Sprite data structure - what cartridges provide to PPU
#[derive(Clone)]
pub struct SpriteData {
    pub x: f32,
    pub y: f32,
    pub sprite_id: u32,
    pub active: bool,
}

pub struct Ppu {
    // Screen buffer - RGBA format
    screen_buffer: Vec<u8>,

    // PPU registers (authentic 8-bit hardware)
    control: u8,
    mask: u8,
    status: u8,

    // Scroll position (hardware registers)
    scroll_x: f32,
    scroll_y: f32,

    // Current scanline and cycle
    scanline: u16,
    cycle: u16,

    // Frame count
    frame_count: u64,

    // Sprite data provided by cartridge
    sprites: Vec<SpriteData>,

    // Demo mode toggle
    color_test_mode: bool,

    // Intro/interlude screen mode
    intro_mode: bool,
    intro_text: String,
    // Z-Synth piano mode
    zsynth_mode: bool,
    
    // HUD/UI data
    hud_lives: u8,
    player_dying: bool,
    player_death_flash: bool,
    player_invulnerable: bool,
    player_invul_flash: bool,
}

impl Ppu {
    pub fn new() -> Ppu {
        let screen_buffer = vec![0; SCREEN_WIDTH * SCREEN_HEIGHT * 4];

        Ppu {
            screen_buffer,
            control: 0,
            mask: 0,
            status: 0,
            scroll_x: 0.0,
            scroll_y: 0.0,
            scanline: 0,
            cycle: 0,
            frame_count: 0,
            sprites: Vec::new(),
            color_test_mode: false,
            intro_mode: false,
            intro_text: String::new(),
            zsynth_mode: false,
            hud_lives: 3,
            player_dying: false,
            player_death_flash: false,
            player_invulnerable: false,
            player_invul_flash: false,
        }
    }

    pub fn step(&mut self, _memory: &Memory) -> bool {
        self.cycle += 1;

        // Simple scanline progression
        if self.cycle >= 341 {
            self.cycle = 0;
            self.scanline += 1;

            if self.scanline >= 262 {
                self.scanline = 0;
                self.frame_count += 1;
                return true; // Frame complete
            }
        }

        false
    }

    // Hardware register access for cartridges
    pub fn set_scroll(&mut self, x: f32, y: f32) {
        self.scroll_x = x;
        self.scroll_y = y;
    }

    pub fn get_scroll_x(&self) -> f32 {
        self.scroll_x
    }

    pub fn get_scroll_y(&self) -> f32 {
        self.scroll_y
    }

    // Sprite management - cartridge provides sprite data
    pub fn clear_sprites(&mut self) {
        self.sprites.clear();
    }

    pub fn add_sprite(&mut self, x: f32, y: f32, sprite_id: u32, active: bool) {
        self.sprites.push(SpriteData {
            x,
            y,
            sprite_id,
            active,
        });
    }

    // Color test mode (debugging)
    pub fn toggle_color_test(&mut self) {
        self.color_test_mode = !self.color_test_mode;
    }

    pub fn get_color_test_mode(&self) -> bool {
        self.color_test_mode
    }

    // Intro/interlude screen mode
    pub fn set_intro_mode(&mut self, intro_mode: bool) {
        self.intro_mode = intro_mode;
    }

    pub fn set_intro_text(&mut self, text: String) {
        self.intro_text = text;
    }

    pub fn set_zsynth_mode(&mut self, zsynth_mode: bool) {
        self.zsynth_mode = zsynth_mode;
    }

    pub fn set_lives(&mut self, lives: u32) {
        self.hud_lives = lives as u8;
    }

    pub fn set_player_death_state(&mut self, is_dying: bool, should_flash: bool) {
        self.player_dying = is_dying;
        self.player_death_flash = should_flash;
    }

    pub fn set_player_invulnerability_state(&mut self, is_invulnerable: bool, should_flash: bool) {
        self.player_invulnerable = is_invulnerable;
        self.player_invul_flash = should_flash;
    }

    // Rendering
    pub fn render(&mut self) {
        if self.color_test_mode {
            self.render_color_test();
        } else if self.intro_mode {
            self.render_intro_screen();
        } else if self.zsynth_mode {
            self.render_zsynth_screen();
        } else {
            self.render_game();
        }
    }

    fn render_game(&mut self) {
        // Clear screen with background color
        let bg_color = MASTER_PALETTE[0]; // Black
        for i in (0..self.screen_buffer.len()).step_by(4) {
            self.screen_buffer[i] = bg_color.0;     // R
            self.screen_buffer[i + 1] = bg_color.1; // G
            self.screen_buffer[i + 2] = bg_color.2; // B
            self.screen_buffer[i + 3] = 255;        // A
        }

        // Render background patterns
        self.render_background();

        // Render sprites provided by cartridge
        let sprites = self.sprites.clone();
        let scroll_x = self.scroll_x;
        let scroll_y = self.scroll_y;

        // Render sprites provided by cartridge
        for sprite in &sprites {
            if sprite.active {
                self.render_sprite(sprite.x - scroll_x, sprite.y - scroll_y, sprite.sprite_id);
            }
        }

        // Render lives counter
        self.render_lives_counter();

        // Debug: Render coordinate display
        self.render_debug_coordinates();
    }

    fn render_background(&mut self) {
        // Render sky gradient
        self.render_sky_gradient();

        // Render mountain layers with parallax
        self.render_mountains();

        // Render background trees between mountains and foreground
        self.render_background_trees();

        // Render ground terrain
        self.render_ground_terrain();
    }

    fn render_sky_gradient(&mut self) {
        // Create a vertical gradient from light blue (top) to lighter blue/white (bottom)
        for y in 0..SCREEN_HEIGHT {
            // Calculate gradient position (0.0 at top, 1.0 at bottom)
            let gradient_pos = y as f32 / SCREEN_HEIGHT as f32;

            // Sky gradient: bright blue at top, lighter towards horizon
            let palette_index = if gradient_pos < 0.3 {
                84u8  // Bright blue
            } else if gradient_pos < 0.6 {
                85u8  // Slightly lighter blue
            } else if gradient_pos < 0.8 {
                86u8  // Even lighter blue
            } else {
                87u8  // Light blue near horizon
            };

            let color = MASTER_PALETTE[palette_index as usize % MASTER_PALETTE.len()];

            // Fill the entire width with this color
            for x in 0..SCREEN_WIDTH {
                let pixel_index = (y * SCREEN_WIDTH + x) * 4;
                self.screen_buffer[pixel_index] = color.0;
                self.screen_buffer[pixel_index + 1] = color.1;
                self.screen_buffer[pixel_index + 2] = color.2;
                self.screen_buffer[pixel_index + 3] = 255;
            }
        }
    }

    fn render_mountains(&mut self) {
        // Parallax mountain silhouettes in the background
        let mountain_parallax_factor = 0.3; // Mountains move 30% of camera speed
        let mountain_offset = -self.scroll_x * mountain_parallax_factor;

        // Render mountain layers (back to front)
        self.render_mountain_layer(mountain_offset * 0.5, 100, 96u8);  // Far mountains (purple)
        self.render_mountain_layer(mountain_offset * 0.7, 120, 80u8);  // Mid mountains (darker blue)
        self.render_mountain_layer(mountain_offset, 140, 48u8);        // Near mountains (dark green)
    }

    fn render_mountain_layer(&mut self, offset: f32, base_height: usize, color_index: u8) {
        let color = MASTER_PALETTE[color_index as usize % MASTER_PALETTE.len()];

        // Create mountain silhouette using a simple sin wave pattern
        for x in 0..SCREEN_WIDTH {
            let world_x = x as f32 + self.scroll_x + offset;

            // Create mountain profile using multiple sin waves for natural look
            let mountain_height =
                ((world_x * 0.01).sin() * 30.0) +           // Large mountains
                ((world_x * 0.03).sin() * 15.0) +           // Medium hills
                ((world_x * 0.05).sin() * 8.0) +            // Small details
                ((world_x * 0.02).cos() * 20.0);            // Add some asymmetry

            let mountain_top = (base_height as f32 + mountain_height) as usize;

            // Fill from mountain top to bottom of screen
            for y in mountain_top..SCREEN_HEIGHT {
                if y < SCREEN_HEIGHT {
                    let pixel_index = (y * SCREEN_WIDTH + x) * 4;
                    // Blend with existing color for transparency effect
                    let existing_r = self.screen_buffer[pixel_index];
                    let existing_g = self.screen_buffer[pixel_index + 1];
                    let existing_b = self.screen_buffer[pixel_index + 2];

                    // Simple alpha blending (50% mountain, 50% sky)
                    self.screen_buffer[pixel_index] = ((color.0 as u16 + existing_r as u16) / 2) as u8;
                    self.screen_buffer[pixel_index + 1] = ((color.1 as u16 + existing_g as u16) / 2) as u8;
                    self.screen_buffer[pixel_index + 2] = ((color.2 as u16 + existing_b as u16) / 2) as u8;
                    self.screen_buffer[pixel_index + 3] = 255;
                }
            }
        }
    }

    fn render_ground_terrain(&mut self) {
        // Render ground level terrain that scrolls with camera
        let ground_level = 200; // Base ground level in world coordinates

        for x in 0..SCREEN_WIDTH {
            let world_x = x as f32 + self.scroll_x;

            // Create slight terrain variation
            let terrain_height = ((world_x * 0.02).sin() * 5.0) as i32;
            let world_ground_y = ground_level + terrain_height;

            // Convert world coordinates to screen coordinates
            let screen_ground_y = (world_ground_y as f32 - self.scroll_y) as i32;

            // Render ground from terrain level to bottom of screen
            for screen_y in screen_ground_y.max(0)..SCREEN_HEIGHT as i32 {
                if screen_y >= 0 && screen_y < SCREEN_HEIGHT as i32 {
                    let pixel_index = (screen_y as usize * SCREEN_WIDTH + x) * 4;

                    // Ground color based on depth from surface
                    let depth = screen_y - screen_ground_y;
                    let ground_color = if depth < 5 {
                        49u8  // Bright green grass
                    } else if depth < 15 {
                        33u8  // Brown dirt
                    } else {
                        17u8  // Dark brown rock
                    };

                    let color = MASTER_PALETTE[ground_color as usize % MASTER_PALETTE.len()];
                    self.screen_buffer[pixel_index] = color.0;
                    self.screen_buffer[pixel_index + 1] = color.1;
                    self.screen_buffer[pixel_index + 2] = color.2;
                    self.screen_buffer[pixel_index + 3] = 255;
                }
            }
        }
    }

    fn render_background_trees(&mut self) {
        // Render stylized background trees with parallax
        let tree_parallax = 0.4; // Trees move slower than foreground
        let tree_offset = -self.scroll_x * tree_parallax;

        // Place trees at regular intervals
        for tree_pos in (0..1200).step_by(80) {
            let tree_x = (tree_pos as f32 + tree_offset) % (SCREEN_WIDTH as f32 + 100.0) - 50.0;
            let ground_y = 200.0 + ((tree_x * 0.02).sin() * 5.0); // Follow ground contour

            self.render_single_tree(tree_x as i32, ground_y as i32);
        }
    }

    fn render_single_tree(&mut self, base_x: i32, base_y: i32) {
        // Simple tree silhouette - trunk and crown
        let trunk_width = 4;
        let trunk_height = 25;
        let crown_radius = 15;

        // Render trunk
        let trunk_color = MASTER_PALETTE[32 % MASTER_PALETTE.len()]; // Brown
        for y in (base_y - trunk_height)..base_y {
            for x in (base_x - trunk_width / 2)..(base_x + trunk_width / 2) {
                if x >= 0 && x < SCREEN_WIDTH as i32 && y >= 0 && y < SCREEN_HEIGHT as i32 {
                    let pixel_index = ((y as usize * SCREEN_WIDTH) + x as usize) * 4;
                    if pixel_index + 3 < self.screen_buffer.len() {
                        self.screen_buffer[pixel_index] = trunk_color.0;
                        self.screen_buffer[pixel_index + 1] = trunk_color.1;
                        self.screen_buffer[pixel_index + 2] = trunk_color.2;
                        self.screen_buffer[pixel_index + 3] = 255;
                    }
                }
            }
        }

        // Render crown (circular)
        let crown_color = MASTER_PALETTE[48 % MASTER_PALETTE.len()]; // Dark green
        let crown_center_y = base_y - trunk_height - crown_radius / 2;

        for y in (crown_center_y - crown_radius)..(crown_center_y + crown_radius) {
            for x in (base_x - crown_radius)..(base_x + crown_radius) {
                if x >= 0 && x < SCREEN_WIDTH as i32 && y >= 0 && y < SCREEN_HEIGHT as i32 {
                    let dist_sq = (x - base_x) * (x - base_x) + (y - crown_center_y) * (y - crown_center_y);
                    if dist_sq <= (crown_radius * crown_radius) {
                        let pixel_index = ((y as usize * SCREEN_WIDTH) + x as usize) * 4;
                        if pixel_index + 3 < self.screen_buffer.len() {
                            // Add some variation to the crown shape
                            let variation = ((x as f32 * 0.3).sin() + (y as f32 * 0.4).cos()) * 0.3;
                            if variation > -0.2 { // Create irregular crown edge
                                self.screen_buffer[pixel_index] = crown_color.0;
                                self.screen_buffer[pixel_index + 1] = crown_color.1;
                                self.screen_buffer[pixel_index + 2] = crown_color.2;
                                self.screen_buffer[pixel_index + 3] = 255;
                            }
                        }
                    }
                }
            }
        }
    }

    fn render_sprite(&mut self, x: f32, y: f32, sprite_id: u32) {
        // Get sprite dimensions based on sprite type
        let (sprite_width, sprite_height) = match sprite_id {
            0 => (32, 28),  // Hambert (larger)
            1 => (32, 16),  // Platform
            2 => (24, 24),  // Enemy
            6 => (16, 16),  // Hamberry
            3 => (20, 32),  // Ninja
            4 => (12, 12),  // Shuriken
            5 => (12, 12),  // Small Hambert head (for lives counter)
            10 => (25, 80), // White piano key (unpressed)
            11 => (25, 80), // White piano key (pressed)
            12 => (15, 50), // Black piano key (unpressed)
            13 => (15, 50), // Black piano key (pressed)
            _ => (32, 32),  // Default
        };

        for py in 0..sprite_height {
            for px in 0..sprite_width {
                let screen_x = x as i32 + px as i32;
                let screen_y = y as i32 + py as i32;

                if screen_x >= 0 && screen_x < SCREEN_WIDTH as i32 &&
                   screen_y >= 0 && screen_y < SCREEN_HEIGHT as i32 {

                    let color_index = self.get_sprite_pixel(sprite_id, px, py);
                    if color_index > 0 {
                        let mut color = MASTER_PALETTE[color_index as usize % MASTER_PALETTE.len()];
                        
                        // Apply death flash effect for player sprite (sprite_id 0)
                        if sprite_id == 0 && self.player_dying {
                            if self.player_death_flash {
                                // Flash white
                                color = (255, 255, 255);
                            } else {
                                // Flash red
                                color = (255, 100, 100);
                            }
                        }
                        // Apply invulnerability flash effect for player sprite (sprite_id 0)
                        else if sprite_id == 0 && self.player_invulnerable {
                            if self.player_invul_flash {
                                // Make semi-transparent (skip rendering this pixel)
                                continue;
                            } else {
                                // Normal rendering
                            }
                        }
                        
                        let buffer_index = (screen_y as usize * SCREEN_WIDTH + screen_x as usize) * 4;

                        if buffer_index + 3 < self.screen_buffer.len() {
                            self.screen_buffer[buffer_index] = color.0;
                            self.screen_buffer[buffer_index + 1] = color.1;
                            self.screen_buffer[buffer_index + 2] = color.2;
                            self.screen_buffer[buffer_index + 3] = 255;
                        }
                    }
                }
            }
        }
    }

    fn get_sprite_pixel(&self, sprite_id: u32, x: u32, y: u32) -> u8 {
        match sprite_id {
            0 => self.get_new_hambert_pixel(x, y),  // Player/Hambert (new improved sprite)
            1 => self.get_platform_pixel(x, y),     // Platform
            2 => self.get_enemy_pixel(x, y),        // Basic enemy
            3 => self.get_ninja_pixel(x, y),        // Ninja
            4 => self.get_shuriken_pixel(x, y),     // Shuriken
            5 => self.get_small_hambert_head_pixel(x, y), // Small Hambert head
            6 => self.get_hamberry_pixel(x, y),     // Hamberry collectible
            10 => self.get_white_piano_key_pixel(x, y, false), // White key unpressed
            11 => self.get_white_piano_key_pixel(x, y, true),  // White key pressed
            12 => self.get_black_piano_key_pixel(x, y, false), // Black key unpressed
            13 => self.get_black_piano_key_pixel(x, y, true),  // Black key pressed
            _ => 0, // Transparent for unknown sprites
        }
    }

    fn get_hambert_pixel(&self, x: u32, y: u32) -> u8 {
        // Scale from 32x28 back to original 24x20 sprite data
        if x >= 32 || y >= 28 {
            return 0; // Transparent outside bounds
        }
        
        // Scale coordinates to original sprite size
        let orig_x = (x * 24) / 32;  // Scale from 32 to 24
        let orig_y = (y * 20) / 28;  // Scale from 28 to 20

        // Original pixel data array from hambertBoy.js (24x20, scaled to 32x28)
        let pixel_data = [
            [0,0,0,0,0,0,0,1,1,1,1,1,1,1,1,1,0,0,0,0,0,0,0,0],
            [0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,0,0,0,0],
            [0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,0],
            [0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
            [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
            [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
            [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
            [1,1,1,2,2,2,1,1,1,1,1,1,1,1,1,2,2,2,1,1,1,1,1,1],
            [1,1,1,2,3,2,1,1,1,1,1,1,1,1,1,2,3,2,1,1,1,1,1,1],
            [1,1,1,1,1,1,1,1,1,1,4,4,4,1,1,1,1,1,1,1,1,1,1,1],
            [1,1,1,1,1,1,1,1,1,4,4,4,4,4,1,1,1,1,1,1,1,1,1,1],
            [1,1,1,1,1,1,1,1,1,1,4,4,4,1,1,1,1,1,1,1,1,1,1,1],
            [0,0,1,1,1,1,1,1,1,1,1,6,6,1,1,1,1,1,1,1,1,1,0,0],
            [0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0],
            [0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,0],
            [0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,0],
            [0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,0,0],
            [0,0,0,0,7,7,7,7,7,0,0,0,0,7,7,7,7,7,0,0,0,0,0,0],
            [0,0,0,0,5,5,5,5,5,5,0,0,5,5,5,5,5,5,0,0,0,0,0,0],
            [0,0,0,0,5,5,5,5,5,5,0,0,5,5,5,5,5,5,0,0,0,0,0,0],
        ];

        // Use scaled coordinates to access original pixel data
        let pixel = pixel_data[orig_y as usize][orig_x as usize];
        match pixel {
            0 => 0,   // transparent
            1 => 10,  // mid-light gray fur
            2 => 0,   // black eye outline
            3 => 15,  // white eye
            4 => 0,   // black nose
            5 => 16,  // red boots
            6 => 120, // pink tongue
            7 => 32,  // brown boot tops
            _ => 0,   // transparent fallback
        }
    }

    fn get_platform_pixel(&self, x: u32, y: u32) -> u8 {
        // Detailed platform texture like original
        if y < 2 {
            // Top grass layer with variation
            if (x + y) % 3 == 0 {
                52 // Bright green grass
            } else {
                49 // Medium green grass
            }
        } else if y < 6 {
            // Dirt layer with some texture
            if (x + y) % 4 == 0 {
                34 // Lighter brown dirt
            } else {
                33 // Medium brown dirt
            }
        } else if y < 10 {
            // Deeper dirt
            if (x + y) % 5 == 0 {
                32 // Dark brown
            } else {
                17 // Very dark brown
            }
        } else {
            // Rock layer at bottom
            if (x + y) % 6 == 0 {
                8 // Dark gray rock
            } else {
                1 // Very dark gray/black rock
            }
        }
    }

    fn get_enemy_pixel(&self, x: u32, y: u32) -> u8 {
        // Simple 24x24 enemy sprite (red)
        if x < 2 || x >= 22 || y < 2 || y >= 22 {
            0 // Black border
        } else {
            16 // Red from palette
        }
    }

    fn get_ninja_pixel(&self, x: u32, y: u32) -> u8 {
        // 20x32 ninja sprite
        if y < 8 {
            // Head area
            if (x >= 6 && x < 14) && (y >= 1 && y < 7) {
                if (x == 7 || x == 12) && (y == 3 || y == 4) {
                    return 15; // Eyes (white)
                }
                return 1; // Head (black/dark)
            }
            return 0;
        } else if y >= 8 && y < 20 {
            // Body/torso
            if x >= 5 && x < 15 {
                if y >= 10 && y < 16 {
                    return 8; // Dark gray ninja outfit
                }
                if (x >= 3 && x < 6) || (x >= 14 && x < 17) {
                    return 8; // Arms
                }
                return 1; // Black outfit
            }
            if ((x >= 1 && x < 4) || (x >= 16 && x < 19)) && (y >= 12 && y < 16) {
                return 8; // Extended arms
            }
            return 0;
        } else {
            // Legs
            if (x >= 6 && x < 8) || (x >= 12 && x < 14) {
                return 8; // Legs
            }
            if y >= 29 && ((x >= 4 && x < 9) || (x >= 11 && x < 16)) {
                return 0; // Black feet/shoes
            }
            return 0;
        }
    }

    fn get_shuriken_pixel(&self, x: u32, y: u32) -> u8 {
        // 12x12 spinning shuriken - simplified for now
        let center_x = 6.0;
        let center_y = 6.0;
        let dx = x as f32 - center_x;
        let dy = y as f32 - center_y;
        let distance = (dx * dx + dy * dy).sqrt();

        if distance <= 4.0 && distance >= 1.0 {
            if distance <= 2.8 {
                7  // Light gray center
            } else {
                0  // Black edges
            }
        } else {
            0 // Transparent
        }
    }

    fn render_debug_coordinates(&mut self) {
        // Show world coordinates at each corner
        let text_color = MASTER_PALETTE[15]; // White

        // Top-left: (scroll_x, scroll_y)
        let tl_text = format!("({:.0},{:.0})", self.scroll_x, self.scroll_y);
        self.render_text(&tl_text, 2, 2, text_color);

        // Top-right: (scroll_x + 320, scroll_y)
        let tr_text = format!("({:.0},{:.0})", self.scroll_x + 320.0, self.scroll_y);
        self.render_text(&tr_text, 250, 2, text_color);

        // Bottom-left: (scroll_x, scroll_y + 240)
        let bl_text = format!("({:.0},{:.0})", self.scroll_x, self.scroll_y + 240.0);
        self.render_text(&bl_text, 2, 230, text_color);

        // Bottom-right: (scroll_x + 320, scroll_y + 240)
        let br_text = format!("({:.0},{:.0})", self.scroll_x + 320.0, self.scroll_y + 240.0);
        self.render_text(&br_text, 250, 230, text_color);
    }

    fn render_text(&mut self, text: &str, x: usize, y: usize, color: (u8, u8, u8)) {
        // Simple text rendering using the font
        for (i, ch) in text.chars().enumerate() {
            if ch.is_ascii() {
                let char_index = (ch as u8).saturating_sub(32) as usize;
                if char_index < FONT_8X8.len() {
                    self.render_char(char_index, x + i * 8, y, color);
                }
            }
        }
    }

    fn render_char(&mut self, char_index: usize, x: usize, y: usize, color: (u8, u8, u8)) {
        let font_data = FONT_8X8[char_index];

        for row in 0..8 {
            let byte = font_data[row];
            for col in 0..8 {
                if (byte >> col) & 1 != 0 {
                    let pixel_x = x + col;
                    let pixel_y = y + row;

                    if pixel_x < SCREEN_WIDTH && pixel_y < SCREEN_HEIGHT {
                        let pixel_index = (pixel_y * SCREEN_WIDTH + pixel_x) * 4;
                        if pixel_index + 3 < self.screen_buffer.len() {
                            self.screen_buffer[pixel_index] = color.0;
                            self.screen_buffer[pixel_index + 1] = color.1;
                            self.screen_buffer[pixel_index + 2] = color.2;
                            self.screen_buffer[pixel_index + 3] = 255;
                        }
                    }
                }
            }
        }
    }

    fn render_color_test(&mut self) {
        // Render color test pattern
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let color_index = ((x / 16) + (y / 16) * 20) % MASTER_PALETTE.len();
                let color = MASTER_PALETTE[color_index];

                let buffer_index = (y * SCREEN_WIDTH + x) * 4;
                self.screen_buffer[buffer_index] = color.0;
                self.screen_buffer[buffer_index + 1] = color.1;
                self.screen_buffer[buffer_index + 2] = color.2;
                self.screen_buffer[buffer_index + 3] = 255;
            }
        }
    }

    fn render_intro_screen(&mut self) {
        // Clear screen with dark blue background
        let bg_color = MASTER_PALETTE[82]; // Dark blue from palette
        for i in (0..self.screen_buffer.len()).step_by(4) {
            self.screen_buffer[i] = bg_color.0;     // R
            self.screen_buffer[i + 1] = bg_color.1; // G
            self.screen_buffer[i + 2] = bg_color.2; // B
            self.screen_buffer[i + 3] = 255;        // A
        }

        // Render large Hambert sprite in center of screen
        let sprite_scale = 3; // Make it 3x larger (96x84 pixels)
        let sprite_x = (SCREEN_WIDTH as i32 - 32 * sprite_scale) / 2;
        let sprite_y = 50; // Position it in upper portion of screen

        self.render_large_hambert_sprite(sprite_x, sprite_y, sprite_scale);

        // Render intro text below the sprite
        let text_y = sprite_y + 32 * sprite_scale + 20; // Below the large sprite (32 is new height)
        let text_color = MASTER_PALETTE[15]; // White
        self.render_intro_text(text_y, text_color);
    }

    fn get_small_hambert_head_pixel(&self, x: u32, y: u32) -> u8 {
        // Small 12x12 version of Hambert's head for lives counter
        if x >= 12 || y >= 12 {
            return 0; // Transparent outside bounds
        }

        // Extract just the head portion (rows 1-12) from the full sprite and scale down
        // This is a simplified version of Hambert's head
        let head_data = [
            [0,0,0,1,1,1,1,1,1,0,0,0],  // Top of head outline
            [0,0,1,9,8,8,1,1,9,1,0,0],  // Head with some facial features
            [0,1,9,9,9,9,9,10,9,9,1,0], // More head detail
            [0,1,9,8,8,9,10,8,8,9,1,0], // Eyes area
            [1,9,9,10,9,7,9,10,9,9,8,1], // More facial features
            [9,10,10,10,9,9,9,9,11,11,9,0], // Face shading
            [9,9,10,9,9,7,8,7,7,9,8,0], // Nose/mouth area
            [8,9,7,7,7,7,7,7,7,7,7,8], // Lower face
            [7,10,11,10,10,10,10,10,10,8,8,0], // Chin area
            [10,9,7,10,10,10,10,10,10,10,9,8], // Lower head
            [10,9,7,10,8,8,8,10,10,8,9,0], // Jaw line
            [11,7,7,10,8,10,8,8,10,7,9,11], // Bottom of head
        ];

        head_data[y as usize][x as usize]
    }

    fn render_lives_counter(&mut self) {
        // Get lives count from the HUD register (set by cartridge)
        let lives = self.hud_lives as u32;
        let start_x = 10; // 10 pixels from left edge
        let start_y = 10; // 10 pixels from top edge
        let spacing = 20; // 20 pixels between each life icon

        for i in 0..lives {
            let x = start_x + (i * spacing);
            self.render_sprite(x as f32, start_y as f32, 5); // Sprite ID 5 is small Hambert head
        }
    }

    fn get_new_hambert_pixel(&self, x: u32, y: u32) -> u8 {
        // New improved Hambert idle sprite data (30x32) - version 2 with better outlines
        if x >= 30 || y >= 32 {
            return 0; // Transparent outside bounds
        }

        let pixel_data = [
            [0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0],
            [0,0,0,0,0,0,0,0,0,0,0,1,1,1,9,8,8,1,1,1,0,0,0,0,0,0,0,0,0,0],
            [0,0,0,0,0,0,0,0,0,0,0,0,9,9,9,9,9,10,9,0,0,0,0,0,0,0,0,0,0,0],
            [0,0,0,0,0,0,0,0,0,0,0,9,9,9,8,8,9,10,8,8,0,0,0,0,0,0,0,0,0,0],
            [0,0,0,0,0,0,0,0,0,0,10,9,9,10,9,7,9,10,9,9,8,0,0,0,0,0,0,0,0,0],
            [0,0,0,0,0,0,0,0,0,9,10,10,10,10,9,9,9,9,11,11,9,9,0,0,0,0,0,0,0,0],
            [0,0,0,0,0,0,0,0,0,9,9,10,9,9,9,7,8,7,7,9,8,9,0,0,0,0,0,0,0,0],
            [0,0,0,0,0,0,0,0,8,9,7,7,7,7,7,7,7,7,7,7,7,8,0,0,0,0,0,0,0,0],
            [0,0,0,0,0,0,0,9,7,10,11,10,10,10,10,10,10,10,10,10,10,8,8,0,0,0,0,0,0,0],
            [0,0,0,0,0,0,0,10,9,7,10,10,10,10,10,10,10,10,10,10,10,10,9,8,0,0,0,0,0,0],
            [0,0,0,0,0,0,10,10,9,7,10,10,8,8,8,10,10,10,8,8,8,10,10,8,9,0,0,0,0,0],
            [0,0,0,0,0,11,10,11,7,7,10,8,8,10,8,8,10,10,8,10,8,8,10,7,9,11,0,0,0,0],
            [0,0,0,0,9,10,10,10,7,10,10,0,0,0,0,8,9,8,0,0,0,0,10,7,7,11,11,0,0,0],
            [0,0,0,0,10,10,11,7,7,10,10,9,14,0,15,11,8,12,15,0,15,8,10,9,7,10,11,0,0,0],
            [0,0,0,10,10,11,10,7,10,10,10,10,15,15,15,11,10,10,15,15,15,10,10,7,7,9,11,0,0,0],
            [0,0,0,10,10,10,11,7,10,10,10,10,10,9,10,10,12,10,9,9,9,10,10,7,7,10,11,0,0,0],
            [0,0,0,0,11,9,8,7,10,10,10,10,10,10,9,8,0,9,9,9,10,10,11,7,6,9,0,0,0,0],
            [0,0,0,0,0,0,0,5,10,10,10,10,10,9,9,0,0,0,10,9,10,10,10,0,0,0,0,0,0,0],
            [0,0,0,0,0,0,0,7,10,9,9,10,10,10,12,0,0,0,11,10,11,10,10,0,0,0,0,0,0,0],
            [0,0,0,0,0,0,0,7,10,10,10,10,10,10,0,7,5,10,0,10,10,9,0,0,0,0,0,0,0,0],
            [0,0,0,0,0,0,0,7,10,10,10,10,10,0,10,12,12,9,10,0,10,9,0,0,0,0,0,0,0,0],
            [0,0,0,0,0,0,0,7,7,10,11,10,10,10,7,12,12,9,10,10,10,10,0,0,0,0,0,0,0,0],
            [0,0,0,0,0,0,0,0,7,10,10,10,10,11,8,27,12,9,10,10,10,10,0,0,0,0,0,0,0,0],
            [0,0,0,0,0,0,0,0,7,7,7,7,10,10,10,10,10,10,10,10,9,8,0,0,0,0,0,0,0,0],
            [0,0,0,0,0,0,0,0,0,18,18,18,18,10,10,10,10,10,10,8,7,18,0,0,0,0,0,0,0,0],
            [0,0,0,0,0,0,0,0,0,18,18,18,18,6,5,4,6,7,6,7,7,18,0,0,0,0,0,0,0,0],
            [0,0,0,0,0,0,0,0,0,18,18,18,18,10,8,7,6,6,6,18,18,18,18,0,0,0,0,0,0,0],
            [0,0,0,0,0,0,0,0,0,18,18,18,18,18,18,18,18,0,18,18,18,18,18,18,0,0,0,0,0,0],
            [0,0,0,0,0,0,0,0,18,18,18,18,18,18,18,18,18,18,0,18,18,18,18,18,18,18,0,0,0,0],
            [0,0,0,0,0,0,0,18,18,18,18,18,18,18,18,18,18,18,18,0,18,18,18,18,18,18,18,0,0,0],
            [0,0,0,0,0,0,0,18,18,18,18,18,18,18,18,18,18,18,18,18,0,18,18,18,18,18,18,0,0,0],
            [0,0,0,0,0,0,0,18,18,18,18,18,18,18,18,18,18,18,18,18,0,18,18,18,18,18,18,0,0,0],
        ];

        // Return the pixel value directly (already mapped to correct palette indices)
        pixel_data[y as usize][x as usize]
    }

    fn get_hamberry_pixel(&self, x: u32, y: u32) -> u8 {
        // 16x16 hamberry sprite data
        if x >= 16 || y >= 16 {
            return 0; // Transparent outside bounds
        }

        let pixel_data = [
            [0,0,0,0,1,1,0,0,0,0,1,1,0,0,0,0],
            [0,0,1,1,1,61,1,1,0,1,61,61,1,0,0,0],
            [0,0,1,62,63,4,62,4,60,62,4,63,1,1,0,0],
            [0,0,0,3,63,63,60,0,0,60,63,63,1,0,0,0],
            [0,0,0,0,2,61,3,1,1,3,60,1,1,0,0,0],
            [0,0,0,0,16,16,1,21,21,1,16,16,0,0,0,0],
            [0,0,0,16,16,1,1,17,17,1,2,16,0,0,0,0],
            [0,0,0,16,0,28,17,0,0,18,28,0,16,16,0,0],
            [0,0,16,16,16,21,22,2,16,22,21,16,28,16,0,0],
            [0,0,16,17,2,21,21,0,0,21,21,1,17,16,0,0],
            [0,0,16,2,1,1,1,28,28,1,1,1,16,16,0,0],
            [0,0,16,16,18,1,28,22,22,16,1,18,16,0,0,0],
            [0,0,0,16,16,18,1,17,17,1,18,16,16,0,0,0],
            [0,0,0,0,16,16,0,2,2,0,16,16,0,0,0,0],
            [0,0,0,0,0,16,16,16,21,16,16,0,0,0,0,0],
            [0,0,0,0,0,0,0,16,16,16,0,0,0,0,0,0],
        ];

        // Return the pixel value directly (already mapped to correct palette indices)
        pixel_data[y as usize][x as usize]
    }

    fn render_large_hambert_sprite(&mut self, base_x: i32, base_y: i32, scale: i32) {
        // Render scaled up Hambert sprite using the new improved sprite data
        for py in 0..32 { // New sprite height (30x32)
            for px in 0..30 { // New sprite width
                let color_index = self.get_new_hambert_pixel(px, py);
                if color_index > 0 { // Only render non-transparent pixels
                    let color = MASTER_PALETTE[color_index as usize % MASTER_PALETTE.len()];

                    // Scale up the pixel by drawing a scale x scale block
                    for sy in 0..scale {
                        for sx in 0..scale {
                            let screen_x = base_x + (px as i32 * scale) + sx;
                            let screen_y = base_y + (py as i32 * scale) + sy;

                            if screen_x >= 0 && screen_x < SCREEN_WIDTH as i32 &&
                               screen_y >= 0 && screen_y < SCREEN_HEIGHT as i32 {
                                let buffer_index = (screen_y as usize * SCREEN_WIDTH + screen_x as usize) * 4;
                                if buffer_index + 3 < self.screen_buffer.len() {
                                    self.screen_buffer[buffer_index] = color.0;
                                    self.screen_buffer[buffer_index + 1] = color.1;
                                    self.screen_buffer[buffer_index + 2] = color.2;
                                    self.screen_buffer[buffer_index + 3] = 255;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn render_intro_text(&mut self, y: i32, color: (u8, u8, u8)) {
        // Clone the text to avoid borrowing issues
        let text = self.intro_text.clone();

        // Don't render anything if text is empty
        if text.is_empty() {
            return;
        }

        // Center the text horizontally
        let text_width = text.len() * 8; // 8 pixels per character
        let text_x = (SCREEN_WIDTH as i32 - text_width as i32) / 2;

        // Make sure the position is valid
        let safe_x = text_x.max(0) as usize;
        let safe_y = y.max(0) as usize;

        // Render the intro text in a visible position (test with fixed position first)
        self.render_text(&text, 20, 200, color);

        // Also render at the calculated center position
        if safe_y < SCREEN_HEIGHT && safe_x < SCREEN_WIDTH {
            self.render_text(&text, safe_x, safe_y, color);
        }
    }

    fn render_zsynth_screen(&mut self) {
        // Clear screen with dark purple background for Z-Synth
        let bg_color = MASTER_PALETTE[95]; // Dark purple from palette
        for i in (0..self.screen_buffer.len()).step_by(4) {
            self.screen_buffer[i] = bg_color.0;     // R
            self.screen_buffer[i + 1] = bg_color.1; // G
            self.screen_buffer[i + 2] = bg_color.2; // B
            self.screen_buffer[i + 3] = 255;        // A
        }

        // Render test text to verify rendering pipeline
        let title_color = MASTER_PALETTE[15]; // White
        self.render_text("Z-SYNTH PIANO", 110, 20, title_color);
        
        let info_color = MASTER_PALETTE[31]; // Light blue
        self.render_text("TEST RENDERING MODE", 90, 40, info_color);
        self.render_text("KEYS: Z S X D C V G B H N J M", 50, 60, info_color);
        self.render_text("NOTES: C2 through B2", 80, 80, info_color);

        // Render sprites provided by cartridge (piano keys)
        let sprites = self.sprites.clone();
        for sprite in &sprites {
            if sprite.active {
                self.render_sprite(sprite.x, sprite.y, sprite.sprite_id);
            }
        }

        // Debug info
        let debug_color = MASTER_PALETTE[47]; // Yellow
        self.render_text(&format!("Sprites: {}", self.sprites.len()), 10, 200, debug_color);
        self.render_text(&format!("Frame: {}", self.frame_count), 10, 220, debug_color);
    }

    pub fn get_screen_buffer(&self) -> Vec<u8> {
        self.screen_buffer.clone()
    }

    pub fn get_frame_count(&self) -> u64 {
        self.frame_count
    }

    // Piano key rendering methods
    fn get_white_piano_key_pixel(&self, x: u32, y: u32, is_pressed: bool) -> u8 {
        if x >= 25 || y >= 80 {
            return 0; // Transparent outside bounds
        }

        // White piano key design with borders and shading
        let border_thickness = 1;
        let shadow_width = 2;
        
        // Define key regions
        let is_border = x < border_thickness || x >= 25 - border_thickness || 
                       y < border_thickness || y >= 80 - border_thickness;
        let is_right_shadow = x >= 25 - shadow_width;
        let is_bottom_shadow = y >= 80 - shadow_width;
        
        if is_pressed {
            // Pressed key - use red to make it obvious
            if is_border {
                16 // Red
            } else {
                20 // Bright red
            }
        } else {
            // Unpressed key - use white/gray
            if is_border {
                1 // Black border
            } else {
                15 // White main area
            }
        }
    }

    fn get_black_piano_key_pixel(&self, x: u32, y: u32, is_pressed: bool) -> u8 {
        if x >= 15 || y >= 50 {
            return 0; // Transparent outside bounds
        }

        // Black piano key design
        let border_thickness = 1;
        let highlight_width = 1;
        
        let is_border = x < border_thickness || x >= 15 - border_thickness || 
                       y < border_thickness || y >= 50 - border_thickness;
        let is_left_highlight = x < highlight_width;
        let is_top_highlight = y < highlight_width;
        
        if is_pressed {
            // Pressed black key - use bright color to make it obvious  
            24 // Bright yellow when pressed
        } else {
            // Unpressed black key - black
            if is_border {
                3 // Dark gray border
            } else {
                1 // Black main area
            }
        }
    }

}