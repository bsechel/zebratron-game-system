#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;
use crate::memory::Memory;
use crate::font_system::{FontSystem, Language, get_font_data};

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
    pub flip_horizontal: bool,
}

// Sprite with pixel data - for direct rendering
#[derive(Clone)]
pub struct SpriteWithData {
    pub x: f32,
    pub y: f32,
    pub pixel_data: Vec<Vec<u8>>, // Variable-sized sprite with palette indices
    pub active: bool,
    pub flip_horizontal: bool,
    pub palette_cycle: u8, // For energy/palette cycling effects (0-3)
    pub scale: f32, // Scale multiplier (1.0 = normal, 4.0 = giant)
}

pub struct Ppu {
    // Screen buffer - RGBA format
    screen_buffer: Vec<u8>,

    // Runtime palette (mutable for palette cycling effects)
    palette: [(u8, u8, u8); 128],

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
    sprites_with_data: Vec<SpriteWithData>,

    // Demo mode toggle
    color_test_mode: bool,

    // Font system for internationalization
    font_system: FontSystem,

    // Intro/interlude screen mode
    intro_mode: bool,
    intro_text: String,
    // Z-Synth piano mode
    zsynth_mode: bool,
    // Platformer game mode
    platformer_mode: bool,
    // Title screen mode
    title_screen_mode: bool,
    title_logo: Option<Vec<Vec<u8>>>, // 320x240 full-screen title image
    show_press_start: bool,

    // Cutscene mode
    cutscene_mode: bool,
    cutscene_image: Option<Vec<Vec<u8>>>, // 64x64 cutscene image
    cutscene_text: Vec<String>, // Text lines for cutscene
    cutscene_scroll_offset: f32, // Text scroll offset
    cutscene_char_index: usize, // Current character to display (for typing effect)
    
    // HUD/UI data
    hud_lives: u8,
    hud_hamberries: u32,
    player_dying: bool,
    player_death_flash: bool,
    player_invulnerable: bool,
    player_invul_flash: bool,

    // Platformer level tiles (from cartridge)
    platformer_tiles: Option<Vec<Vec<u8>>>,
    // Platformer tileset pixel data (256 tiles, 16×16 pixels each)
    platformer_tileset: Option<Vec<[[u8; 16]; 16]>>,

    // Pre-rendered hill layers for smooth scrolling
    hill_layer_far: Vec<u8>,    // Far hills height map (width: 1024)
    hill_layer_near: Vec<u8>,   // Near hills height map (width: 1024)
}

impl Ppu {
    pub fn new() -> Ppu {
        let screen_buffer = vec![0; SCREEN_WIDTH * SCREEN_HEIGHT * 4];

        Ppu {
            screen_buffer,
            palette: MASTER_PALETTE, // Initialize with default palette
            control: 0,
            mask: 0,
            status: 0,
            scroll_x: 0.0,
            scroll_y: 0.0,
            scanline: 0,
            cycle: 0,
            frame_count: 0,
            sprites: Vec::new(),
            sprites_with_data: Vec::new(),
            color_test_mode: false,
            font_system: FontSystem::new(),
            intro_mode: false,
            intro_text: String::new(),
            zsynth_mode: false,
            platformer_mode: false,
            title_screen_mode: false,
            title_logo: None,
            show_press_start: false,
            cutscene_mode: false,
            cutscene_image: None,
            cutscene_text: Vec::new(),
            cutscene_scroll_offset: 0.0,
            cutscene_char_index: 0,
            hud_lives: 3,
            hud_hamberries: 0,
            player_dying: false,
            player_death_flash: false,
            player_invulnerable: false,
            player_invul_flash: false,
            platformer_tiles: None,
            platformer_tileset: None,
            hill_layer_far: Self::generate_hill_layer_far(1024),
            hill_layer_near: Self::generate_hill_layer(1024, 1.2, 2.3),
        }
    }

    // Generate far hill layer with more crests and higher peaks
    fn generate_hill_layer_far(width: usize) -> Vec<u8> {
        use std::f32::consts::PI;
        let mut layer = Vec::with_capacity(width);

        for x in 0..width {
            // Calculate amplitude scale based on position (0.0 to 1.0 across buffer)
            let progress = (x as f32) / (width as f32);
            let amplitude_scale = 0.2 + (progress * 0.8); // Start at 20%, grow to 100%

            let world_x = (x as f32) * 0.01;
            // More waves with higher amplitude for dramatic far mountains
            let height = 40.0 +
                        20.0 * amplitude_scale * (world_x * 0.6).sin() +   // Main rolling wave
                        12.0 * amplitude_scale * (world_x * 1.3).sin() +   // Secondary crests
                        6.0 * amplitude_scale * (world_x * 2.7).sin();     // Fine detail
            // Store as u8 (0-255 range)
            layer.push(height.clamp(0.0, 255.0) as u8);
        }

        // Apply smoothing pass to reduce jaggedness
        let mut smoothed = layer.clone();
        for x in 1..(width - 1) {
            smoothed[x] = ((layer[x - 1] as u16 + layer[x] as u16 * 2 + layer[x + 1] as u16) / 4) as u8;
        }

        smoothed
    }

    // Generate a pre-rendered hill layer with given parameters
    // Hills start flat and grow in amplitude across the buffer
    fn generate_hill_layer(width: usize, freq1: f32, freq2: f32) -> Vec<u8> {
        use std::f32::consts::PI;
        let mut layer = Vec::with_capacity(width);

        for x in 0..width {
            // Calculate amplitude scale based on position (0.0 to 1.0 across buffer)
            let progress = (x as f32) / (width as f32);
            let amplitude_scale = 0.2 + (progress * 0.8); // Start at 20%, grow to 100%

            let world_x = (x as f32) * 0.01; // Reduced frequency for smoother hills
            let height = 30.0 +
                        15.0 * amplitude_scale * (world_x * freq1).sin() +
                        8.0 * amplitude_scale * (world_x * freq2).sin();
            // Store as u8 (0-255 range)
            layer.push(height.clamp(0.0, 255.0) as u8);
        }

        // Apply smoothing pass to reduce jaggedness
        let mut smoothed = layer.clone();
        for x in 1..(width - 1) {
            smoothed[x] = ((layer[x - 1] as u16 + layer[x] as u16 * 2 + layer[x + 1] as u16) / 4) as u8;
        }

        smoothed
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
        self.sprites_with_data.clear();
    }

    pub fn add_sprite(&mut self, x: f32, y: f32, sprite_id: u32, active: bool, flip_horizontal: bool) {
        self.sprites.push(SpriteData {
            x,
            y,
            sprite_id,
            active,
            flip_horizontal,
        });
    }
    
    pub fn add_sprite_with_data(&mut self, x: f32, y: f32, pixel_data: &[Vec<u8>], active: bool, flip_horizontal: bool, scale: f32) {
        self.add_sprite_with_data_and_cycle(x, y, pixel_data, active, flip_horizontal, 0, scale);
    }

    pub fn add_sprite_with_data_and_cycle(&mut self, x: f32, y: f32, pixel_data: &[Vec<u8>], active: bool, flip_horizontal: bool, palette_cycle: u8, scale: f32) {
        self.sprites_with_data.push(SpriteWithData {
            x,
            y,
            pixel_data: pixel_data.to_vec(),
            active,
            flip_horizontal,
            palette_cycle,
            scale,
        });
    }

    // Color test mode (debugging)
    pub fn toggle_color_test(&mut self) {
        self.color_test_mode = !self.color_test_mode;
    }

    pub fn get_color_test_mode(&self) -> bool {
        self.color_test_mode
    }

    // Palette cycling for animated effects (energy, water, lava, etc.)
    pub fn cycle_palette_range(&mut self, start: usize, end: usize) {
        // Rotate colors in the specified range (inclusive)
        // Example: cycle_palette_range(120, 127) rotates indices 120-127
        if start >= end || end >= self.palette.len() {
            return; // Invalid range
        }

        let temp = self.palette[start];
        for i in start..end {
            self.palette[i] = self.palette[i + 1];
        }
        self.palette[end] = temp;
    }

    pub fn reset_palette(&mut self) {
        // Reset to original master palette
        self.palette = MASTER_PALETTE;
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

    pub fn set_platformer_mode(&mut self, platformer_mode: bool) {
        self.platformer_mode = platformer_mode;
    }

    pub fn set_platformer_tiles(&mut self, tiles: Vec<Vec<u8>>) {
        self.platformer_tiles = Some(tiles);
    }

    pub fn set_platformer_tileset(&mut self, tileset: Vec<[[u8; 16]; 16]>) {
        self.platformer_tileset = Some(tileset);
    }

    pub fn set_title_screen_mode(&mut self, title_screen_mode: bool) {
        self.title_screen_mode = title_screen_mode;
    }

    pub fn set_title_logo(&mut self, logo: Vec<Vec<u8>>) {
        self.title_logo = Some(logo);
    }

    pub fn set_show_press_start(&mut self, show: bool) {
        self.show_press_start = show;
    }

    pub fn set_cutscene_mode(&mut self, cutscene_mode: bool) {
        self.cutscene_mode = cutscene_mode;
    }

    pub fn set_cutscene_image(&mut self, image: Vec<Vec<u8>>) {
        self.cutscene_image = Some(image);
    }

    pub fn set_cutscene_text(&mut self, text: Vec<String>) {
        self.cutscene_text = text;
    }

    pub fn set_cutscene_scroll_offset(&mut self, offset: f32) {
        self.cutscene_scroll_offset = offset;
    }

    pub fn set_cutscene_char_index(&mut self, index: usize) {
        self.cutscene_char_index = index;
    }

    pub fn set_lives(&mut self, lives: u32) {
        self.hud_lives = lives as u8;
    }

    pub fn set_hamberries(&mut self, hamberries: u32) {
        self.hud_hamberries = hamberries;
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
        } else if self.cutscene_mode {
            self.render_cutscene();
        } else if self.title_screen_mode {
            self.render_title_screen();
        } else if self.platformer_mode {
            self.render_platformer();
        } else {
            self.render_game();
        }
    }

    fn render_game(&mut self) {
        // Clear screen with background color
        let bg_color = self.palette[0]; // Black
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
                self.render_sprite(sprite.x - scroll_x, sprite.y - scroll_y, sprite.sprite_id, sprite.flip_horizontal);
            }
        }

        // Render lives counter
        self.render_lives_counter();

        // Debug: Render coordinate display
        self.render_debug_coordinates();
    }

    fn render_platformer(&mut self) {
        // Clear screen with a different background color (sky)
        let bg_color = self.palette[65]; // Sky color
        for i in (0..self.screen_buffer.len()).step_by(4) {
            self.screen_buffer[i] = bg_color.0;     // R
            self.screen_buffer[i + 1] = bg_color.1; // G
            self.screen_buffer[i + 2] = bg_color.2; // B
            self.screen_buffer[i + 3] = 255;        // A
        }

        // Render simple platformer background
        self.render_platformer_background();

        // Render tiles from the platformer cartridge with scroll offset
        self.render_platformer_tiles();

        // Render sprites provided by cartridge (player)
        let sprites = self.sprites.clone();
        let sprites_with_data = self.sprites_with_data.clone();
        let scroll_x = self.scroll_x;
        let scroll_y = self.scroll_y;

        // Render old-style sprites (with sprite_id)
        for sprite in &sprites {
            if sprite.active {
                self.render_platformer_sprite(sprite.x - scroll_x, sprite.y - scroll_y, sprite.sprite_id, sprite.flip_horizontal);
            }
        }
        
        // Render new-style sprites (with pixel data)
        for sprite in &sprites_with_data {
            if sprite.active {
                self.render_sprite_with_data(sprite.x - scroll_x, sprite.y - scroll_y, &sprite.pixel_data, sprite.flip_horizontal, sprite.palette_cycle, sprite.scale);
            }
        }

        // Render lives hearts in top-right corner
        self.render_hearts();

        // Render Hamberry count in HUD (next to the icon at x:18, y:10)
        let count_text = format!("x{}", self.hud_hamberries);
        self.render_text(&count_text, 20, 11, (255, 255, 255));

        // Render debug coordinates
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

            let color = self.palette[palette_index as usize % self.palette.len()];

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
        let color = self.palette[color_index as usize % self.palette.len()];

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

                    let color = self.palette[ground_color as usize % self.palette.len()];
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
        let trunk_color = self.palette[32 % self.palette.len()]; // Brown
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
        let crown_color = self.palette[48 % self.palette.len()]; // Dark green
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

    fn render_sprite(&mut self, x: f32, y: f32, sprite_id: u32, flip_horizontal: bool) {
        // Get sprite dimensions based on sprite type
        let (sprite_width, sprite_height) = match sprite_id {
            0 => (32, 28),  // Hambert (larger)
            1 => (32, 16),  // Platform
            2 => (24, 24),  // Enemy
            6 => (16, 16),  // Hamberry
            3 => (20, 28),  // Hexagnome (scaled down for performance)
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

                    // Apply horizontal flipping if needed
                    let sprite_px = if flip_horizontal {
                        sprite_width - 1 - px
                    } else {
                        px
                    };
                    
                    let color_index = self.get_sprite_pixel(sprite_id, sprite_px, py);
                    if color_index > 0 {
                        let mut color = self.palette[color_index as usize % self.palette.len()];
                        
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
            3 => self.get_hexagnome_pixel(x, y),    // Hexagnome
            4 => self.get_shuriken_pixel(x, y),     // Shuriken
            5 => self.get_small_hambert_head_pixel(x, y), // Small Hambert head
            6 => self.get_hamberry_pixel(x, y),     // Hamberry collectible
            7 => self.get_blood_goblin_pixel(x, y), // Blood Goblin
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

    fn get_hexagnome_pixel(&self, x: u32, y: u32) -> u8 {
        // 20x28 hexagnome sprite - full bitmap with scaling
        if x >= 20 || y >= 28 {
            return 0; // Transparent outside bounds
        }
        
        // Scale coordinates to original sprite size for lookup
        let orig_x = (x * 26) / 20;  // Scale from 20 to 26
        let orig_y = (y * 32) / 28;  // Scale from 28 to 32
        
        if orig_x >= 26 || orig_y >= 32 {
            return 0;
        }
        
        // Original hexagnome pixel data - kept as static for performance
        static HEXAGNOME_PIXEL_DATA: [[u8; 26]; 32] = [
            [0,0,0,0,0,0,0,0,1,3,3,4,4,3,3,0,0,0,0,0,0,0,0,0,0,0],
            [0,0,0,0,0,0,0,3,0,3,3,4,4,3,3,2,1,0,0,0,0,0,0,0,0,0],
            [0,0,0,0,0,0,0,1,6,5,2,3,3,3,2,2,2,4,0,0,0,0,0,0,0,0],
            [0,0,0,0,0,0,0,8,13,7,2,2,2,2,2,4,4,4,3,0,0,0,0,0,0,0],
            [0,0,0,0,0,0,0,13,11,11,4,2,2,3,4,5,4,4,3,0,0,0,0,0,0,0],
            [0,0,0,0,0,0,11,10,10,9,3,0,2,3,2,2,2,4,5,4,0,0,0,0,0,0],
            [0,0,0,0,0,9,12,9,3,1,1,1,1,2,1,2,5,4,5,5,3,0,0,0,0,0],
            [0,0,0,0,0,0,4,1,1,1,1,1,2,2,3,5,6,5,4,5,4,4,0,0,0,0],
            [0,0,0,0,0,0,0,0,0,0,1,1,2,3,5,5,6,5,4,5,4,3,0,0,0,0],
            [0,0,0,0,0,0,0,0,0,0,0,1,2,4,5,5,6,4,4,6,5,2,2,0,0,0],
            [0,0,0,0,0,0,0,0,0,3,1,1,3,4,5,6,6,4,4,5,6,2,2,0,0,0],
            [0,0,0,0,0,0,0,0,3,1,1,1,3,5,5,6,5,3,4,5,5,3,3,0,0,0],
            [0,0,0,0,0,0,3,3,3,1,0,1,3,5,5,6,5,3,4,4,5,3,3,0,0,0],
            [0,3,3,3,3,3,3,3,3,1,2,3,3,4,5,6,3,3,4,4,6,3,2,0,0,0],
            [0,3,13,13,12,3,3,3,2,3,3,3,4,4,4,4,3,3,4,5,4,1,0,1,0,0],
            [0,0,3,13,12,3,3,3,3,3,3,3,4,3,3,4,4,4,4,5,5,3,2,1,0,0],
            [0,0,3,3,3,0,0,3,3,3,3,4,3,2,3,5,4,4,4,5,4,3,5,3,0,0],
            [0,0,0,0,0,0,3,3,3,3,2,2,1,1,4,5,4,4,4,4,4,4,2,3,3,0],
            [0,0,0,0,0,2,3,3,2,2,1,1,1,1,2,4,4,4,4,4,4,4,2,2,3,0],
            [0,0,0,0,0,5,2,2,1,0,1,2,3,3,3,3,4,4,4,4,4,3,2,2,3,0],
            [0,0,0,0,3,12,12,8,1,2,2,3,4,4,3,3,4,4,4,4,4,3,2,2,3,0],
            [0,0,0,0,3,13,13,7,3,3,3,4,4,4,3,3,3,4,4,4,3,3,2,3,2,0],
            [0,0,0,0,3,3,3,3,3,3,3,3,4,5,4,4,4,4,4,3,3,2,2,3,3,0],
            [0,0,0,0,0,0,0,0,3,3,3,2,3,4,5,5,4,4,3,3,3,2,1,3,0,0],
            [0,0,0,0,0,0,0,0,3,3,4,3,2,2,3,4,4,3,3,3,3,2,1,3,0,0],
            [0,0,0,0,0,0,0,4,3,3,4,5,4,3,3,3,4,3,3,3,2,2,3,3,0,0],
            [0,0,0,0,0,0,0,3,3,3,4,4,4,4,4,3,3,3,3,3,2,2,3,3,0,0],
            [0,0,0,0,0,0,0,3,3,2,3,4,4,4,4,3,3,3,3,3,2,0,0,0,0,0],
            [0,0,0,0,0,0,0,0,0,0,3,3,3,3,3,3,3,3,3,3,0,0,0,0,0,0],
            [0,0,0,0,0,0,0,0,0,3,3,3,3,3,2,3,3,3,3,3,0,0,0,0,0,0],
            [0,0,0,0,0,0,0,0,3,3,3,3,3,2,3,3,3,3,3,3,3,0,0,0,0,0],
            [0,0,0,0,0,0,3,3,3,3,3,3,2,3,3,3,3,3,3,3,3,0,0,0,0,0],
        ];

        HEXAGNOME_PIXEL_DATA[orig_y as usize][orig_x as usize]
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

    fn get_blood_goblin_pixel(&self, x: u32, y: u32) -> u8 {
        // 20x38 blood goblin sprite using actual converted data
        if x >= 20 || y >= 38 {
            return 0; // Transparent outside bounds
        }
        
        // Blood goblin pixel data (20x38) - using the actual converted sprite data
        static BLOOD_GOBLIN_PIXEL_DATA: [[u8; 20]; 38] = [
            [0,0,0,0,0,0,122,122,0,0,0,0,0,0,0,0,0,0,0,0],
            [0,0,0,0,0,0,45,5,0,0,0,0,0,0,0,0,0,0,0,0],
            [0,0,0,0,0,5,30,18,17,17,6,0,0,0,0,0,0,0,0,0],
            [0,0,0,0,0,8,42,45,19,18,18,45,0,0,0,0,0,0,0,0],
            [0,0,0,0,0,7,45,9,8,45,18,45,0,0,0,0,0,0,0,0],
            [0,0,0,0,0,4,41,41,45,45,30,6,0,0,0,0,0,0,0,0],
            [0,0,0,0,0,4,28,28,4,5,8,0,0,0,0,0,0,0,0,0],
            [0,0,0,0,0,44,44,28,4,4,44,0,0,0,0,0,0,0,0,0],
            [0,0,0,0,28,45,17,17,18,44,4,17,45,8,0,0,0,0,0,0],
            [0,0,47,9,44,30,7,18,18,45,45,45,8,8,0,0,0,0,0,0],
            [0,47,42,3,45,46,45,18,45,45,45,44,5,42,8,0,0,0,0,0],
            [47,41,4,0,8,45,45,45,45,29,18,110,6,45,41,0,0,0,0,0],
            [47,6,0,0,110,30,45,45,18,18,17,0,0,0,41,9,0,0,0,0],
            [9,6,0,0,110,45,45,45,45,18,45,0,0,0,41,9,0,0,0,0],
            [0,41,6,0,8,45,45,18,18,45,29,0,0,3,8,0,0,0,0,0],
            [0,7,8,0,110,18,18,29,18,29,23,0,4,41,0,0,0,0,0,0],
            [0,0,8,7,45,18,29,18,18,18,23,8,3,0,0,0,0,0,0,0],
            [0,0,10,7,6,28,18,18,18,18,44,4,8,0,0,0,0,0,0,0],
            [0,0,0,11,28,17,18,18,18,44,4,8,0,0,0,0,0,0,0,0],
            [0,0,0,0,18,18,18,18,18,16,8,0,0,0,0,0,0,0,0,0],
            [0,0,0,111,18,18,18,18,18,18,25,0,0,0,0,0,0,0,0,0],
            [0,0,0,45,18,18,18,18,18,18,0,0,0,0,0,0,0,0,0,0],
            [0,0,0,17,18,18,18,18,17,45,0,0,0,0,0,0,0,0,0,0],
            [0,0,0,45,17,18,18,18,17,25,0,0,0,0,0,0,0,0,0,0],
            [0,0,0,0,18,17,18,17,18,0,0,0,0,0,0,0,0,0,0,0],
            [0,0,0,0,0,17,17,17,17,0,0,0,0,0,0,0,0,0,0,0],
            [0,0,0,0,0,0,110,17,45,17,17,0,0,0,0,0,0,0,0,0],
            [0,0,0,0,0,0,45,17,111,25,17,45,0,0,0,0,0,0,0,0],
            [0,0,0,0,0,0,17,17,0,0,0,45,0,0,0,0,0,0,0,0],
            [0,0,0,0,0,0,17,45,0,0,0,10,110,0,0,0,0,0,0,0],
            [0,0,0,0,0,0,17,9,0,0,0,0,0,0,0,0,0,0,0,0],
            [0,0,0,0,0,45,17,0,0,0,0,0,0,0,0,0,0,0,0,0],
            [0,0,0,0,0,16,46,0,0,0,0,0,0,0,0,0,0,0,0,0],
            [0,0,0,0,0,16,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
            [0,0,0,0,45,16,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
            [0,0,0,0,16,17,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
            [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
            [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        ];
        
        BLOOD_GOBLIN_PIXEL_DATA[y as usize][x as usize]
    }

    fn render_debug_coordinates(&mut self) {
        // Show world coordinates at each corner
        let text_color = self.palette[15]; // White

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

    pub fn render_debug_pixel(&mut self, x: usize, y: usize, color: (u8, u8, u8)) {
        if x < SCREEN_WIDTH && y < SCREEN_HEIGHT {
            let pixel_index = (y * SCREEN_WIDTH + x) * 4;
            if pixel_index + 3 < self.screen_buffer.len() {
                self.screen_buffer[pixel_index] = color.0;     // R
                self.screen_buffer[pixel_index + 1] = color.1; // G
                self.screen_buffer[pixel_index + 2] = color.2; // B
                self.screen_buffer[pixel_index + 3] = 255;     // A
                
                // Debug pixel set successfully
            }
        }
    }

    fn render_text(&mut self, text: &str, x: usize, y: usize, color: (u8, u8, u8)) {
        // Multi-language text rendering using the font system
        let characters = self.font_system.encode_text(text);

        for (i, character) in characters.iter().enumerate() {
            if let Some(font_data) = get_font_data(character.glyph_index) {
                self.render_char_data(font_data, x + i * 8, y, color);
            }
        }
    }

    // Reusable typing effect - renders text character by character
    // Use this for cutscenes, dialogue boxes, menus, etc.
    fn render_text_typing(&mut self, text: &str, x: usize, y: usize, color: (u8, u8, u8), chars_visible: usize) {
        // Only show characters up to chars_visible
        let display_text: String = text.chars().take(chars_visible).collect();
        self.render_text(&display_text, x, y, color);
    }

    // Render hearts for lives display
    fn render_hearts(&mut self) {
        // Heart sprite data will be passed from cartridge
        // For now, render simple colored hearts using palette colors
        // Pink heart colors: 17 (dark pink), 18 (bright pink)

        let hearts_to_render = self.hud_lives.min(10); // Cap at 10 hearts for display
        let heart_spacing = 10; // 8 pixels wide + 2 pixels spacing
        let start_x = SCREEN_WIDTH - (hearts_to_render as usize * heart_spacing) - 10;
        let start_y = 10;

        for i in 0..hearts_to_render {
            let heart_x = start_x + (i as usize * heart_spacing);
            self.render_heart(heart_x, start_y);
        }
    }

    fn render_heart(&mut self, x: usize, y: usize) {
        // 8x8 heart sprite with pink colors (palette indices 17 and 18)
        let heart_pattern = [
            [255,17,17,255,255,17,17,255],
            [17,18,18,17,17,18,18,17],
            [18,18,18,18,18,18,18,18],
            [18,18,18,18,18,18,18,18],
            [17,18,18,18,18,18,18,17],
            [255,17,18,18,18,18,17,255],
            [255,255,17,18,18,17,255,255],
            [255,255,255,17,17,255,255,255],
        ];

        for (row, pattern_row) in heart_pattern.iter().enumerate() {
            for (col, &palette_index) in pattern_row.iter().enumerate() {
                if palette_index == 255 {
                    continue; // Skip transparent pixels
                }

                let pixel_x = x + col;
                let pixel_y = y + row;

                if pixel_x < SCREEN_WIDTH && pixel_y < SCREEN_HEIGHT {
                    let idx = (pixel_y * SCREEN_WIDTH + pixel_x) * 4;
                    if idx < self.screen_buffer.len() - 3 {
                        let color = self.palette[palette_index as usize];
                        self.screen_buffer[idx] = color.0;
                        self.screen_buffer[idx + 1] = color.1;
                        self.screen_buffer[idx + 2] = color.2;
                        self.screen_buffer[idx + 3] = 255;
                    }
                }
            }
        }
    }

    // Set the language for text rendering
    pub fn set_language(&mut self, language: Language) {
        self.font_system.set_language(language);
    }

    fn render_char_data(&mut self, font_data: &[u8; 8], x: usize, y: usize, color: (u8, u8, u8)) {

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

    fn render_small_text(&mut self, text: &str, x: usize, y: usize, color: (u8, u8, u8)) {
        // Small text rendering at 6 pixels tall (6x5 pixels per character)
        for (i, ch) in text.chars().enumerate() {
            if ch.is_ascii() {
                let char_index = (ch as u8).saturating_sub(32) as usize;
                if char_index < FONT_8X8.len() {
                    self.render_small_char(char_index, x + i * 5, y, color);
                }
            }
        }
    }

    fn render_small_char(&mut self, char_index: usize, x: usize, y: usize, color: (u8, u8, u8)) {
        let font_data = FONT_8X8[char_index];

        // Render at 6 pixels tall by scaling 8x8 to 6x5
        for row in 0..6 {
            // Map 6 rows to 8 rows of original font
            let font_row = (row * 8) / 6;
            let byte = font_data[font_row];
            
            for col in 0..5 {
                // Map 5 columns to 8 columns of original font
                let font_col = (col * 8) / 5;
                if (byte >> font_col) & 1 != 0 {
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
        // Render color test pattern organized by families, bigger squares, no titles
        const SQUARE_SIZE: usize = 20; // Bigger squares to fill screen better
        
        // Clear screen with dark background
        let bg_color = self.palette[0]; // Black
        for i in (0..self.screen_buffer.len()).step_by(4) {
            self.screen_buffer[i] = bg_color.0;
            self.screen_buffer[i + 1] = bg_color.1;
            self.screen_buffer[i + 2] = bg_color.2;
            self.screen_buffer[i + 3] = 255;
        }

        // Render organized color families without labels
        let mut current_y = 5;

        // Grays (0-15)
        self.render_color_family_grid(0, 16, 5, current_y, SQUARE_SIZE);
        current_y += SQUARE_SIZE + 5;

        // Reds (16-31) 
        self.render_color_family_grid(16, 16, 5, current_y, SQUARE_SIZE);
        current_y += SQUARE_SIZE + 5;

        // Oranges/Browns (32-47)
        self.render_color_family_grid(32, 16, 5, current_y, SQUARE_SIZE);
        current_y += SQUARE_SIZE + 5;

        // Greens (48-63)
        self.render_color_family_grid(48, 16, 5, current_y, SQUARE_SIZE);
        current_y += SQUARE_SIZE + 5;

        // Cyans (64-79)
        self.render_color_family_grid(64, 16, 5, current_y, SQUARE_SIZE);
        current_y += SQUARE_SIZE + 5;

        // Blues (80-95)
        self.render_color_family_grid(80, 16, 5, current_y, SQUARE_SIZE);
        current_y += SQUARE_SIZE + 5;

        // Purples (96-111)
        self.render_color_family_grid(96, 16, 5, current_y, SQUARE_SIZE);
        current_y += SQUARE_SIZE + 5;

        // Extended colors (112-127)
        self.render_color_family_grid(112, 16, 5, current_y, SQUARE_SIZE);
    }

    fn render_color_family_grid(&mut self, start_index: usize, count: usize, x: usize, y: usize, square_size: usize) {
        // Render color squares without labels
        for i in 0..count {
            let color_index = start_index + i;
            if color_index < self.palette.len() {
                let square_x = x + i * square_size;
                let square_y = y;

                // Render color square
                let color = self.palette[color_index];
                for py in 0..square_size {
                    for px in 0..square_size {
                        let screen_x = square_x + px;
                        let screen_y = square_y + py;
                        
                        if screen_x < SCREEN_WIDTH && screen_y < SCREEN_HEIGHT {
                            let buffer_index = (screen_y * SCREEN_WIDTH + screen_x) * 4;
                            if buffer_index + 3 < self.screen_buffer.len() {
                                self.screen_buffer[buffer_index] = color.0;
                                self.screen_buffer[buffer_index + 1] = color.1;
                                self.screen_buffer[buffer_index + 2] = color.2;
                                self.screen_buffer[buffer_index + 3] = 255;
                            }
                        }
                    }
                }

                // Add color number
                let bg_color = self.palette[color_index];
                let brightness = (bg_color.0 as u32 + bg_color.1 as u32 + bg_color.2 as u32) / 3;
                let text_color = if brightness > 128 { 
                    self.palette[0] // Black for bright backgrounds
                } else { 
                    self.palette[15] // White for dark backgrounds
                };
                
                self.render_small_text(&color_index.to_string(), square_x + 2, square_y + 2, text_color);
            }
        }
    }

    fn render_intro_screen(&mut self) {
        // Clear screen with dark blue background
        let bg_color = self.palette[82]; // Dark blue from palette
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
        let text_color = self.palette[15]; // White
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
        let spacing = 20; // 20 pixels between each heart (bigger hearts need more space)

        for i in 0..lives {
            let x = start_x + (i * spacing);
            self.render_pink_heart_text(x as f32, start_y as f32);
        }
    }

    fn render_pink_heart_text(&mut self, x: f32, y: f32) {
        // Render a big heart pattern in light red
        let heart_color = self.palette[104]; // Color index 104 as requested
        
        // 16x16 heart pattern (double size)
        let heart_pattern = [
            0b0000000000000000, // ................
            0b0000000000000000, // ................
            0b0011110001111000, // ..####...####...
            0b0111111011111100, // .######.######..
            0b1111111111111110, // ##############..
            0b1111111111111110, // ##############..
            0b1111111111111110, // ##############..
            0b1111111111111110, // ##############..
            0b0111111111111100, // .############...
            0b0011111111111000, // ..##########....
            0b0001111111110000, // ...########.....
            0b0000111111100000, // ....######......
            0b0000011111000000, // .....####.......
            0b0000001110000000, // ......##........
            0b0000000100000000, // .......#........
            0b0000000000000000, // ................
        ];

        for (row, &pattern) in heart_pattern.iter().enumerate() {
            for col in 0..16 {
                if (pattern & (1 << (15 - col))) != 0 {
                    let screen_x = (x as usize + col).min(SCREEN_WIDTH - 1);
                    let screen_y = (y as usize + row).min(SCREEN_HEIGHT - 1);
                    let pixel_index = ((screen_y * SCREEN_WIDTH) + screen_x) * 4;
                    
                    if pixel_index + 3 < self.screen_buffer.len() {
                        self.screen_buffer[pixel_index] = heart_color.0;
                        self.screen_buffer[pixel_index + 1] = heart_color.1;
                        self.screen_buffer[pixel_index + 2] = heart_color.2;
                        self.screen_buffer[pixel_index + 3] = 255; // Alpha
                    }
                }
            }
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
                    let color = self.palette[color_index as usize % self.palette.len()];

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
        let bg_color = self.palette[95]; // Dark purple from palette
        for i in (0..self.screen_buffer.len()).step_by(4) {
            self.screen_buffer[i] = bg_color.0;     // R
            self.screen_buffer[i + 1] = bg_color.1; // G
            self.screen_buffer[i + 2] = bg_color.2; // B
            self.screen_buffer[i + 3] = 255;        // A
        }

        // Render test text to verify rendering pipeline
        let title_color = self.palette[15]; // White
        self.render_text("Z-SYNTH PIANO", 110, 20, title_color);
        
        let info_color = self.palette[31]; // Light blue
        self.render_text("TEST RENDERING MODE", 90, 40, info_color);
        self.render_text("KEYS: Z S X D C V G B H N J M", 50, 60, info_color);
        self.render_text("NOTES: C2 through B2", 80, 80, info_color);

        // Render sprites provided by cartridge (piano keys)
        let sprites = self.sprites.clone();
        for sprite in &sprites {
            if sprite.active {
                self.render_sprite(sprite.x, sprite.y, sprite.sprite_id, sprite.flip_horizontal);
            }
        }

        // Debug info
        let debug_color = self.palette[47]; // Yellow
        self.render_text(&format!("Sprites: {}", self.sprites.len()), 10, 200, debug_color);
        self.render_text(&format!("Frame: {}", self.frame_count), 10, 220, debug_color);
    }

    fn render_title_screen(&mut self) {
        // Render full-screen title image (320x240)
        if let Some(ref logo) = self.title_logo {
            for (y, row) in logo.iter().enumerate() {
                for (x, &palette_idx) in row.iter().enumerate() {
                    if x < SCREEN_WIDTH && y < SCREEN_HEIGHT {
                        let color = self.palette[palette_idx as usize];
                        let idx = (y * SCREEN_WIDTH + x) * 4;
                        self.screen_buffer[idx] = color.0;
                        self.screen_buffer[idx + 1] = color.1;
                        self.screen_buffer[idx + 2] = color.2;
                        self.screen_buffer[idx + 3] = 255;
                    }
                }
            }
        } else {
            // Fallback: Clear screen with dark blue background
            let bg_color = self.palette[82]; // Dark blue
            for i in (0..self.screen_buffer.len()).step_by(4) {
                self.screen_buffer[i] = bg_color.0;     // R
                self.screen_buffer[i + 1] = bg_color.1; // G
                self.screen_buffer[i + 2] = bg_color.2; // B
                self.screen_buffer[i + 3] = 255;        // A
            }
        }

        // Render "PRESS START" text if should be shown (blinking animation)
        if self.show_press_start {
            let press_start_color = self.palette[37]; // Bright yellow
            self.render_text("PRESS START", 110, 170, press_start_color);
        }

        // Add copyright/credit text on top of the image
        let credit_color = self.palette[13]; // Light gray
        self.render_small_text("(C) 2025", 130, 220, credit_color);
    }

    fn render_cutscene(&mut self) {
        // Clear screen with black background
        let bg_color = self.palette[0]; // Black
        for i in (0..self.screen_buffer.len()).step_by(4) {
            self.screen_buffer[i] = bg_color.0;     // R
            self.screen_buffer[i + 1] = bg_color.1; // G
            self.screen_buffer[i + 2] = bg_color.2; // B
            self.screen_buffer[i + 3] = 255;        // A
        }

        // Render cutscene image if available (centered at top)
        if let Some(ref image) = self.cutscene_image {
            let img_x = (SCREEN_WIDTH - 64) / 2; // Center the 64px wide image
            let img_y = 30; // Position near top

            for (y, row) in image.iter().enumerate() {
                for (x, &palette_idx) in row.iter().enumerate() {
                    if palette_idx != 0 { // 0 = transparent
                        let screen_x = img_x + x;
                        let screen_y = img_y + y;

                        if screen_x < SCREEN_WIDTH && screen_y < SCREEN_HEIGHT {
                            let color = self.palette[palette_idx as usize];
                            let idx = (screen_y * SCREEN_WIDTH + screen_x) * 4;
                            self.screen_buffer[idx] = color.0;
                            self.screen_buffer[idx + 1] = color.1;
                            self.screen_buffer[idx + 2] = color.2;
                            self.screen_buffer[idx + 3] = 255;
                        }
                    }
                }
            }
        }

        // Render text lines with typing effect
        let text_color = self.palette[15]; // White
        let start_y = 130; // Start below image
        let line_height = 12;

        // Clone text lines to avoid borrow conflict
        let text_lines = self.cutscene_text.clone();
        let char_index = self.cutscene_char_index;

        // Track how many characters we've processed across all lines
        let mut chars_processed = 0;
        for (i, line) in text_lines.iter().enumerate() {
            let y = start_y + (i * line_height) as usize;

            // Calculate how many characters of this line to show
            let chars_to_show = if char_index > chars_processed {
                (char_index - chars_processed).min(line.len())
            } else {
                0
            };

            if chars_to_show > 0 && y < SCREEN_HEIGHT {
                // Center the text (using full line width for consistent positioning)
                let text_width = line.len() * 8;
                let x = if text_width < SCREEN_WIDTH {
                    (SCREEN_WIDTH - text_width) / 2
                } else {
                    10
                };

                // Use the reusable typing effect function
                self.render_text_typing(line, x, y, text_color, chars_to_show);
            }

            chars_processed += line.len();
        }

        // Add "PRESS BUTTON" prompt at bottom
        let prompt_color = self.palette[37]; // Yellow
        self.render_small_text("PRESS ANY BUTTON", 100, 215, prompt_color);
    }

    fn render_platformer_background(&mut self) {
        // Draw rolling hills in background (with parallax)
        self.render_distant_hills();

        // Simple clouds in sky
        let cloud_color = self.palette[63]; // Light gray

        // Draw simple cloud shapes
        for cloud_x in [50, 150, 250] {
            self.render_platformer_cloud(cloud_x, 30);
        }

        // Simple sun
        let sun_color = self.palette[52]; // Yellow
        for y in 20..35 {
            for x in 280..295 {
                if (x as i32 - 287).pow(2) + (y as i32 - 27).pow(2) < 64 { // Circle formula
                    let idx = (y * SCREEN_WIDTH + x) * 4;
                    if idx < self.screen_buffer.len() - 3 {
                        self.screen_buffer[idx] = sun_color.0;
                        self.screen_buffer[idx + 1] = sun_color.1;
                        self.screen_buffer[idx + 2] = sun_color.2;
                        self.screen_buffer[idx + 3] = 255;
                    }
                }
            }
        }
    }

    fn render_platformer_cloud(&mut self, center_x: usize, center_y: usize) {
        let cloud_color = self.palette[63]; // Light gray

        // Draw a simple cloud shape
        for y in (center_y.saturating_sub(5))..(center_y + 5) {
            for x in (center_x.saturating_sub(15))..(center_x + 15) {
                if y < SCREEN_HEIGHT && x < SCREEN_WIDTH {
                    let dx = x.saturating_sub(center_x) as i32;
                    let dy = y.saturating_sub(center_y) as i32;
                    if dx * dx + dy * dy < 30 { // Cloud shape
                        let idx = (y * SCREEN_WIDTH + x) * 4;
                        if idx < self.screen_buffer.len() - 3 {
                            self.screen_buffer[idx] = cloud_color.0;
                            self.screen_buffer[idx + 1] = cloud_color.1;
                            self.screen_buffer[idx + 2] = cloud_color.2;
                            self.screen_buffer[idx + 3] = 255;
                        }
                    }
                }
            }
        }
    }

    fn render_distant_hills(&mut self) {
        // Get scroll position for parallax effect
        let scroll_x = self.scroll_x as f32;

        // Layer 1: Far distant hills (back layer, slowest parallax)
        let far_color = self.palette[78]; // Back hill color
        let parallax_1 = (scroll_x * 0.1) as usize; // Move at 10% of camera speed

        for x in 0..SCREEN_WIDTH {
            // Sample from pre-rendered buffer with wrapping (simple integer lookup)
            let buffer_x = (x + parallax_1) % self.hill_layer_far.len();
            let height = self.hill_layer_far[buffer_x];
            let hill_y = (180 - height as usize).min(SCREEN_HEIGHT); // Base position

            // Fill from hill peak to bottom of screen
            for y in hill_y..SCREEN_HEIGHT {
                let idx = (y * SCREEN_WIDTH + x) * 4;
                if idx < self.screen_buffer.len() - 3 {
                    self.screen_buffer[idx] = far_color.0;
                    self.screen_buffer[idx + 1] = far_color.1;
                    self.screen_buffer[idx + 2] = far_color.2;
                    self.screen_buffer[idx + 3] = 255;
                }
            }
        }

        // Layer 2: Closer hills (front layer, faster parallax)
        let near_color = self.palette[77]; // Front hill color
        let parallax_2 = (scroll_x * 0.2) as usize; // Move at 20% of camera speed

        for x in 0..SCREEN_WIDTH {
            // Sample from pre-rendered buffer with wrapping (simple integer lookup)
            let buffer_x = (x + parallax_2) % self.hill_layer_near.len();
            let height = self.hill_layer_near[buffer_x];
            let hill_y = (195 - height as usize).min(SCREEN_HEIGHT); // Base position

            // Fill from hill peak to bottom of screen
            for y in hill_y..SCREEN_HEIGHT {
                let idx = (y * SCREEN_WIDTH + x) * 4;
                if idx < self.screen_buffer.len() - 3 {
                    self.screen_buffer[idx] = near_color.0;
                    self.screen_buffer[idx + 1] = near_color.1;
                    self.screen_buffer[idx + 2] = near_color.2;
                    self.screen_buffer[idx + 3] = 255;
                }
            }
        }
    }

    fn render_platformer_tiles(&mut self) {
        // Dynamic tile rendering that handles the expanded 200-tile world
        let ground_color = self.palette[24]; // Brown
        let platform_color = self.palette[31]; // Green
        let pitfall_color = self.palette[5]; // Dark red
        let passage_color = self.palette[39]; // Sky blue (same as background)
        let water_color = self.palette[1]; // Blue
        let swim_through_color = self.palette[17]; // Darker blue
        
        // Get scroll offset for camera movement
        let scroll_x = self.scroll_x;
        let scroll_y = self.scroll_y;
        
        // Calculate which tiles are visible on screen - with proper bounds checking
        let tile_start_x = ((scroll_x / 16.0).floor().max(0.0) as usize).min(200);
        let tile_end_x = (((scroll_x + SCREEN_WIDTH as f32) / 16.0).ceil().max(0.0) as usize + 1).min(200);
        let tile_start_y = ((scroll_y / 16.0).floor().max(0.0) as usize).min(15);
        let tile_end_y = (((scroll_y + SCREEN_HEIGHT as f32) / 16.0).ceil().max(0.0) as usize + 1).min(15);
        
        // Render tiles based on the platformer cartridge pattern
        // This mirrors the logic from platformer_cartridge.rs
        for tile_y in tile_start_y..tile_end_y {
            for tile_x in tile_start_x..tile_end_x {
                // Additional safety check to prevent any potential overflow
                if tile_x >= 200 || tile_y >= 15 {
                    continue;
                }
                
                // Determine tile type based on position (mirrors platformer_cartridge.rs logic)
                let tile_type = self.get_platformer_tile_type_id(tile_x, tile_y);

                // Skip air tiles
                if tile_type == 0 {
                    continue;
                }

                // Render with actual tile pixels if tileset is available
                let should_render_pixels = self.platformer_tileset.is_some()
                    && (tile_type as usize) < self.platformer_tileset.as_ref().unwrap().len();

                if should_render_pixels {
                    // Extract tile pixels before calling render (to avoid borrow checker issues)
                    let tile_pixels = self.platformer_tileset.as_ref().unwrap()[tile_type as usize];
                    self.render_tile_pixels_with_scroll(tile_x, tile_y, &tile_pixels, scroll_x, scroll_y);
                    continue;
                }

                // Fallback to solid colors if tileset not available
                let color = match tile_type {
                    1 => if tile_y >= 12 { ground_color } else { platform_color }, // Solid
                    2 => platform_color, // Platform
                    3 => pitfall_color, // Pitfall
                    4 => continue, // Passage - render as sky (don't draw tile)
                    5 => water_color, // Water
                    6 => swim_through_color, // Swim-through
                    _ => continue, // Unknown - don't render
                };

                self.render_tile_at_with_scroll(tile_x, tile_y, color, scroll_x, scroll_y);
            }
        }
    }
    
    // Helper function that returns the actual tile ID from platformer_cartridge.rs logic
    fn get_platformer_tile_type_id(&self, tile_x: usize, tile_y: usize) -> u8 {
        // Safety bounds check
        if tile_x >= 200 || tile_y >= 15 {
            return 0; // Air
        }

        // If we have tiles from the cartridge, use them
        if let Some(ref tiles) = self.platformer_tiles {
            if tile_y < tiles.len() && tile_x < tiles[tile_y].len() {
                return tiles[tile_y][tile_x];
            }
        }

        // Fallback to air if no tiles available
        0
    }
    fn render_tile_pixels_with_scroll(&mut self, tile_x: usize, tile_y: usize, tile_pixels: &[[u8; 16]; 16], scroll_x: f32, scroll_y: f32) {
        let pixel_x_start = (tile_x * 16) as f32 - scroll_x;
        let pixel_y_start = (tile_y * 16) as f32 - scroll_y;

        for dy in 0..16 {
            for dx in 0..16 {
                let pixel_x = pixel_x_start + dx as f32;
                let pixel_y = pixel_y_start + dy as f32;

                // Only render if pixel is on screen
                if pixel_x >= 0.0 && pixel_x < SCREEN_WIDTH as f32 &&
                   pixel_y >= 0.0 && pixel_y < SCREEN_HEIGHT as f32 {
                    let palette_index = tile_pixels[dy][dx];

                    // Skip transparent pixels (palette index 0 or 255)
                    if palette_index == 0 || palette_index == 255 {
                        continue;
                    }

                    let color = self.palette[palette_index as usize];

                    let idx = ((pixel_y as usize) * SCREEN_WIDTH + (pixel_x as usize)) * 4;
                    if idx < self.screen_buffer.len() - 3 {
                        self.screen_buffer[idx] = color.0;
                        self.screen_buffer[idx + 1] = color.1;
                        self.screen_buffer[idx + 2] = color.2;
                        self.screen_buffer[idx + 3] = 255;
                    }
                }
            }
        }
    }

    fn render_tile_at_with_scroll(&mut self, tile_x: usize, tile_y: usize, color: (u8, u8, u8), scroll_x: f32, scroll_y: f32) {
        let pixel_x_start = (tile_x * 16) as f32 - scroll_x;
        let pixel_y_start = (tile_y * 16) as f32 - scroll_y;

        for dy in 0..16 {
            for dx in 0..16 {
                let pixel_x = pixel_x_start + dx as f32;
                let pixel_y = pixel_y_start + dy as f32;

                // Only render if pixel is on screen
                if pixel_x >= 0.0 && pixel_x < SCREEN_WIDTH as f32 &&
                   pixel_y >= 0.0 && pixel_y < SCREEN_HEIGHT as f32 {
                    let idx = ((pixel_y as usize) * SCREEN_WIDTH + (pixel_x as usize)) * 4;
                    if idx < self.screen_buffer.len() - 3 {
                        self.screen_buffer[idx] = color.0;
                        self.screen_buffer[idx + 1] = color.1;
                        self.screen_buffer[idx + 2] = color.2;
                        self.screen_buffer[idx + 3] = 255;
                    }
                }
            }
        }
    }
    
    fn render_tile_at(&mut self, tile_x: usize, tile_y: usize, color: (u8, u8, u8)) {
        let pixel_x_start = tile_x * 16;
        let pixel_y_start = tile_y * 16;
        
        for dy in 0..16 {
            for dx in 0..16 {
                let pixel_x = pixel_x_start + dx;
                let pixel_y = pixel_y_start + dy;
                
                if pixel_x < SCREEN_WIDTH && pixel_y < SCREEN_HEIGHT {
                    let idx = (pixel_y * SCREEN_WIDTH + pixel_x) * 4;
                    if idx < self.screen_buffer.len() - 3 {
                        self.screen_buffer[idx] = color.0;
                        self.screen_buffer[idx + 1] = color.1;
                        self.screen_buffer[idx + 2] = color.2;
                        self.screen_buffer[idx + 3] = 255;
                    }
                }
            }
        }
    }

    fn render_platformer_sprite(&mut self, x: f32, y: f32, sprite_id: u32, flip_horizontal: bool) {
        // Fallback: render a simple square for old-style sprites
        let player_color = self.palette[47]; // Yellow/Orange
        let outline_color = self.palette[0]; // Black
        
        let sprite_x = x as i32;
        let sprite_y = y as i32;
        
        // Draw player as a simple colored square with black outline
        for dy in -8..8 {
            for dx in -8..8 {
                let pixel_x = sprite_x + dx;
                let pixel_y = sprite_y + dy;
                
                if pixel_x >= 0 && pixel_x < SCREEN_WIDTH as i32 && 
                   pixel_y >= 0 && pixel_y < SCREEN_HEIGHT as i32 {
                    let idx = ((pixel_y as usize) * SCREEN_WIDTH + (pixel_x as usize)) * 4;
                    if idx < self.screen_buffer.len() - 3 {
                        // Black outline
                        if dx.abs() == 7 || dy.abs() == 7 {
                            self.screen_buffer[idx] = outline_color.0;
                            self.screen_buffer[idx + 1] = outline_color.1;
                            self.screen_buffer[idx + 2] = outline_color.2;
                            self.screen_buffer[idx + 3] = 255;
                        } else {
                            // Player color fill
                            self.screen_buffer[idx] = player_color.0;
                            self.screen_buffer[idx + 1] = player_color.1;
                            self.screen_buffer[idx + 2] = player_color.2;
                            self.screen_buffer[idx + 3] = 255;
                        }
                    }
                }
            }
        }
    }
    
    fn render_sprite_with_data(&mut self, x: f32, y: f32, pixel_data: &[Vec<u8>], flip_horizontal: bool, palette_cycle: u8, scale: f32) {
        if pixel_data.is_empty() {
            return;
        }

        let sprite_x = x as i32;
        let sprite_y = y as i32;
        let sprite_height = pixel_data.len();
        let sprite_width = pixel_data[0].len();
        let scale_int = scale as i32; // Convert to integer for pixel repetition

        // Render variable-sized sprite with pixel data from cartridge
        for (row, sprite_row) in pixel_data.iter().enumerate() {
            for (col, &palette_index) in sprite_row.iter().enumerate() {
                // Skip transparent pixels (palette index 0 or 255)
                if palette_index == 0 || palette_index == 255 {
                    continue;
                }

                // Apply palette cycling for energy effects (shifts palette index by cycle amount)
                let cycled_index = if palette_cycle > 0 {
                    // Cycle through green energy colors: 48, 50, 52, 54
                    if palette_index >= 48 && palette_index <= 54 && palette_index % 2 == 0 {
                        // Cycle the green indices
                        let base = 48;
                        let offset = (palette_index - base) / 2; // 0, 1, 2, 3
                        base + (((offset + palette_cycle) % 4) * 2)
                    } else {
                        palette_index
                    }
                } else {
                    palette_index
                };

                // Apply horizontal flipping if needed
                let actual_col = if flip_horizontal {
                    sprite_width - 1 - col // Flip horizontally
                } else {
                    col
                };

                // Calculate base position (with scaling applied to centering offset)
                let scaled_width = (sprite_width as i32) * scale_int;
                let scaled_height = (sprite_height as i32) * scale_int;
                let base_pixel_x = sprite_x + (actual_col as i32 * scale_int) - (scaled_width / 2);
                let base_pixel_y = sprite_y + (row as i32 * scale_int) - (scaled_height / 2);

                // For nearest-neighbor scaling, repeat each pixel scale×scale times
                for dy in 0..scale_int {
                    for dx in 0..scale_int {
                        let pixel_x = base_pixel_x + dx;
                        let pixel_y = base_pixel_y + dy;

                        if pixel_x >= 0 && pixel_x < SCREEN_WIDTH as i32 &&
                           pixel_y >= 0 && pixel_y < SCREEN_HEIGHT as i32 {
                            let idx = ((pixel_y as usize) * SCREEN_WIDTH + (pixel_x as usize)) * 4;
                            if idx < self.screen_buffer.len() - 3 {
                                let color = self.palette[cycled_index as usize];
                                self.screen_buffer[idx] = color.0;
                                self.screen_buffer[idx + 1] = color.1;
                                self.screen_buffer[idx + 2] = color.2;
                                self.screen_buffer[idx + 3] = 255;
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn get_screen_buffer(&self) -> Vec<u8> {
        self.screen_buffer.clone()
    }

    pub fn get_screen_buffer_vec(&self) -> Vec<u8> {
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