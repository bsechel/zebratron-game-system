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

#[derive(Clone, Copy)]
pub enum EntityType {
    Player,
    Enemy,
    Platform,
    Projectile,
    Collectible,
}

#[derive(Clone)]
pub struct Entity {
    pub entity_type: EntityType,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub vel_x: f32,
    pub vel_y: f32,
    pub sprite_id: u32,
    pub active: bool,
    pub on_ground: bool,
    pub health: i32,
    pub animation_frame: u32,
    pub animation_timer: f32,
}

impl Entity {
    pub fn new(entity_type: EntityType, x: f32, y: f32, sprite_id: u32) -> Self {
        let (width, height) = match entity_type {
            EntityType::Player => (32.0, 32.0),
            EntityType::Enemy => (24.0, 24.0),
            EntityType::Platform => (64.0, 16.0),
            EntityType::Projectile => (8.0, 8.0),
            EntityType::Collectible => (16.0, 16.0),
        };

        Entity {
            entity_type,
            x,
            y,
            width,
            height,
            vel_x: 0.0,
            vel_y: 0.0,
            sprite_id,
            active: true,
            on_ground: false,
            health: match entity_type {
                EntityType::Player => 3,
                EntityType::Enemy => 1,
                _ => 1,
            },
            animation_frame: 0,
            animation_timer: 0.0,
        }
    }
}

#[wasm_bindgen]
pub struct Ppu {
    // Screen buffer - RGBA format
    screen_buffer: Vec<u8>,

    // PPU registers
    control: u8,
    mask: u8,
    status: u8,

    // Scroll position
    scroll_x: u8,
    scroll_y: u8,

    // Current scanline and cycle
    scanline: u16,
    cycle: u16,

    // Frame count
    frame_count: u64,

    // Game entities - simple approach for WASM compatibility
    entities: Vec<Entity>,
    player_id: usize,
    sprite_x: f32,
    sprite_y: f32,

    // Camera position for scrolling
    camera_x: f32,
    camera_y: f32,

    // World bounds
    world_width: f32,
    world_height: f32,

    // Demo mode toggle
    color_test_mode: bool,
}

#[wasm_bindgen]
impl Ppu {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Ppu {
        let mut ppu = Ppu {
            screen_buffer: vec![0; SCREEN_WIDTH * SCREEN_HEIGHT * 4], // RGBA
            control: 0,
            mask: 0,
            status: 0,
            scroll_x: 0,
            scroll_y: 0,
            scanline: 0,
            cycle: 0,
            frame_count: 0,
            entities: Vec::new(),
            player_id: 0,
            sprite_x: 100.0,
            sprite_y: 100.0,
            camera_x: 0.0,
            camera_y: 0.0,
            world_width: 2400.0,  // Large scrolling world like Hambert Boy
            world_height: 480.0,
            color_test_mode: false,
        };

        // Create a default player entity
        let player = Entity::new(EntityType::Player, 100.0, 100.0, 0);
        ppu.entities.push(player);
        ppu.player_id = 0;

        // Add some platforms for demonstration
        ppu.entities.push(Entity::new(EntityType::Platform, 200.0, 200.0, 1));
        ppu.entities.push(Entity::new(EntityType::Platform, 400.0, 150.0, 2));
        ppu.entities.push(Entity::new(EntityType::Platform, 600.0, 180.0, 3));
        ppu.entities.push(Entity::new(EntityType::Platform, 800.0, 220.0, 4));

        // Add an enemy for demonstration
        ppu.entities.push(Entity::new(EntityType::Enemy, 300.0, 120.0, 5));

        ppu
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

    pub fn render_frame(&mut self, _memory: &Memory) {
        // Clear screen with background color
        for pixel in self.screen_buffer.chunks_mut(4) {
            pixel[0] = 0x20; // R
            pixel[1] = 0x20; // G
            pixel[2] = 0x40; // B (dark blue background)
            pixel[3] = 255;  // A
        }

        // TODO: Implement tile and sprite rendering
        if self.color_test_mode {
            self.render_color_test();
        } else {
            self.render_test_pattern();
        }
    }

    fn render_test_pattern(&mut self) {
        // Update camera to follow sprite (keep sprite centered on screen)
        self.camera_x = self.sprite_x - (SCREEN_WIDTH as f32 / 2.0) + 16.0; // +16 to center 32x32 sprite
        self.camera_y = self.sprite_y - (SCREEN_HEIGHT as f32 / 2.0) + 16.0;

        // Render infinite scrolling background pattern
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let pixel_index = (y * SCREEN_WIDTH + x) * 4;

                // World coordinates (add camera offset for scrolling)
                let world_x = (x as f32 + self.camera_x) as usize;
                let world_y = (y as f32 + self.camera_y) as usize;

                // Create a repeating tile pattern using palette colors
                let palette_index = if world_x % 32 == 0 || world_y % 32 == 0 {
                    7u8  // Light gray grid lines
                } else if (world_x / 32 + world_y / 32) % 2 == 0 {
                    96u8 // Dark purple from palette
                } else {
                    4u8  // Dark gray from palette
                };

                let (r, g, b) = self.get_palette_color(palette_index);
                self.screen_buffer[pixel_index] = r;
                self.screen_buffer[pixel_index + 1] = g;
                self.screen_buffer[pixel_index + 2] = b;
                self.screen_buffer[pixel_index + 3] = 255;
            }
        }

        // Render all entities
        self.render_entities();

        // Add demo text
        self.render_text_centered("Hi, Chris. This is how text will look ;P", 40, 15); // White text
        self.render_text("Use WASD/Arrow Keys to move", 10, 200, 80); // Blue text
        self.render_text("Press Enter for color test", 10, 210, 48); // Green text
    }

    fn render_character_sprite(&mut self, x: usize, y: usize) {
        // Generate 32x32 sprite procedurally to avoid array counting issues
        for sprite_y in 0..32 {
            for sprite_x in 0..32 {
                let screen_x = x + sprite_x;
                let screen_y = y + sprite_y;

                // Bounds check
                if screen_x >= SCREEN_WIDTH || screen_y >= SCREEN_HEIGHT {
                    continue;
                }

                // Generate a simple 32x32 robot pattern using palette indices
                let mut palette_index = 0u8;

                // Border
                if sprite_x == 0 || sprite_x == 31 || sprite_y == 0 || sprite_y == 31 {
                    palette_index = 0; // Black border
                }
                // Head area (top quarter)
                else if sprite_y >= 2 && sprite_y <= 10 && sprite_x >= 6 && sprite_x <= 25 {
                    palette_index = 84; // Blue from palette
                    // Eyes
                    if (sprite_y >= 4 && sprite_y <= 6) &&
                       ((sprite_x >= 10 && sprite_x <= 12) || (sprite_x >= 19 && sprite_x <= 21)) {
                        palette_index = 15; // White from palette
                    }
                    // Mouth
                    if sprite_y >= 7 && sprite_y <= 8 && sprite_x >= 13 && sprite_x <= 18 {
                        palette_index = 0; // Black from palette
                    }
                }
                // Body area
                else if sprite_y >= 12 && sprite_y <= 25 && sprite_x >= 8 && sprite_x <= 23 {
                    palette_index = 84; // Blue fill
                    // Body details (chest)
                    if sprite_y >= 15 && sprite_y <= 18 && sprite_x >= 14 && sprite_x <= 17 {
                        palette_index = 112; // Skin tone from palette
                    }
                }
                // Arms
                else if sprite_y >= 14 && sprite_y <= 20 &&
                        ((sprite_x >= 2 && sprite_x <= 7) || (sprite_x >= 24 && sprite_x <= 29)) {
                    palette_index = 84; // Blue arms
                }
                // Legs
                else if sprite_y >= 26 && sprite_y <= 30 &&
                        ((sprite_x >= 10 && sprite_x <= 14) || (sprite_x >= 17 && sprite_x <= 21)) {
                    palette_index = 84; // Blue legs
                }

                if palette_index > 0 || (sprite_x == 0 || sprite_x == 31 || sprite_y == 0 || sprite_y == 31) {
                    let pixel_index = (screen_y * SCREEN_WIDTH + screen_x) * 4;
                    let (r, g, b) = self.get_palette_color(palette_index);

                    self.screen_buffer[pixel_index] = r;
                    self.screen_buffer[pixel_index + 1] = g;
                    self.screen_buffer[pixel_index + 2] = b;
                    self.screen_buffer[pixel_index + 3] = 255;
                }
            }
        }
    }

    fn render_entities(&mut self) {
        // Collect entity data to avoid borrowing issues
        let entities_data: Vec<(EntityType, f32, f32, f32, f32, bool)> = self.entities.iter()
            .map(|e| (e.entity_type, e.x, e.y, e.width, e.height, e.active))
            .collect();

        let camera_x = self.camera_x;
        let camera_y = self.camera_y;

        // Render all active entities
        for (entity_type, x, y, width, height, active) in entities_data {
            if !active {
                continue;
            }

            // Calculate screen position relative to camera
            let screen_x = (x - camera_x) as i32;
            let screen_y = (y - camera_y) as i32;

            // Skip if entity is completely off screen
            if screen_x + (width as i32) < 0 || screen_x >= (SCREEN_WIDTH as i32) ||
               screen_y + (height as i32) < 0 || screen_y >= (SCREEN_HEIGHT as i32) {
                continue;
            }

            // Render entity sprite based on type
            match entity_type {
                EntityType::Player => {
                    if screen_x >= 0 && screen_y >= 0 {
                        self.render_character_sprite(screen_x as usize, screen_y as usize);
                    }
                },
                EntityType::Platform => {
                    self.render_platform_sprite(screen_x as i32, screen_y as i32, width as i32, height as i32);
                },
                EntityType::Enemy => {
                    if screen_x >= 0 && screen_y >= 0 {
                        self.render_enemy_sprite(screen_x as usize, screen_y as usize);
                    }
                },
                _ => {
                    // Simple colored rectangle for other entity types
                    self.render_simple_sprite(screen_x as i32, screen_y as i32, width as i32, height as i32, 64); // Cyan
                }
            }
        }
    }

    fn render_platform_sprite(&mut self, x: i32, y: i32, width: i32, height: i32) {
        let color = self.get_palette_color(32); // Brown color for platforms

        for dy in 0..height {
            for dx in 0..width {
                let screen_x = x + dx;
                let screen_y = y + dy;

                if screen_x >= 0 && screen_x < SCREEN_WIDTH as i32 &&
                   screen_y >= 0 && screen_y < SCREEN_HEIGHT as i32 {
                    let pixel_index = ((screen_y as usize * SCREEN_WIDTH) + screen_x as usize) * 4;
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

    fn render_enemy_sprite(&mut self, x: usize, y: usize) {
        // Simple 24x24 enemy sprite (red)
        for sprite_y in 0..24 {
            for sprite_x in 0..24 {
                let screen_x = x + sprite_x;
                let screen_y = y + sprite_y;

                if screen_x >= SCREEN_WIDTH || screen_y >= SCREEN_HEIGHT {
                    continue;
                }

                // Simple red enemy pattern
                let palette_index = if sprite_x < 2 || sprite_x >= 22 || sprite_y < 2 || sprite_y >= 22 {
                    0 // Black border
                } else {
                    16 // Red from palette
                };

                if palette_index > 0 || (sprite_x < 2 || sprite_x >= 22 || sprite_y < 2 || sprite_y >= 22) {
                    let pixel_index = (screen_y * SCREEN_WIDTH + screen_x) * 4;
                    let (r, g, b) = self.get_palette_color(palette_index);
                    self.screen_buffer[pixel_index] = r;
                    self.screen_buffer[pixel_index + 1] = g;
                    self.screen_buffer[pixel_index + 2] = b;
                    self.screen_buffer[pixel_index + 3] = 255;
                }
            }
        }
    }

    fn render_simple_sprite(&mut self, x: i32, y: i32, width: i32, height: i32, color_index: u8) {
        let color = self.get_palette_color(color_index);

        for dy in 0..height {
            for dx in 0..width {
                let screen_x = x + dx;
                let screen_y = y + dy;

                if screen_x >= 0 && screen_x < SCREEN_WIDTH as i32 &&
                   screen_y >= 0 && screen_y < SCREEN_HEIGHT as i32 {
                    let pixel_index = ((screen_y as usize * SCREEN_WIDTH) + screen_x as usize) * 4;
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

    pub fn get_screen_buffer(&self) -> js_sys::Uint8Array {
        js_sys::Uint8Array::from(&self.screen_buffer[..])
    }

    pub fn get_screen_width(&self) -> u32 {
        SCREEN_WIDTH as u32
    }

    pub fn get_screen_height(&self) -> u32 {
        SCREEN_HEIGHT as u32
    }

    pub fn move_sprite(&mut self, dx: f32, dy: f32) {
        if self.player_id < self.entities.len() {
            let player = &mut self.entities[self.player_id];
            player.x += dx;
            player.y += dy;
            // Keep player within world bounds
            player.x = player.x.max(0.0).min(self.world_width - player.width);
            player.y = player.y.max(0.0).min(self.world_height - player.height);

            // Also update sprite_x/y for backward compatibility
            self.sprite_x = player.x;
            self.sprite_y = player.y;

            // Update camera to follow player
            self.update_camera_follow_player();
        }
    }

    fn update_camera_follow_player(&mut self) {
        if self.player_id < self.entities.len() {
            let player = &self.entities[self.player_id];
            let screen_center_x = SCREEN_WIDTH as f32 / 2.0;
            let screen_center_y = SCREEN_HEIGHT as f32 / 2.0;

            // Center camera on player
            self.camera_x = player.x + player.width / 2.0 - screen_center_x;
            self.camera_y = player.y + player.height / 2.0 - screen_center_y;

            // Keep camera within world bounds
            self.camera_x = self.camera_x.max(0.0).min(self.world_width - SCREEN_WIDTH as f32);
            self.camera_y = self.camera_y.max(0.0).min(self.world_height - SCREEN_HEIGHT as f32);
        }
    }

    pub fn add_entity(&mut self, entity_type: u32, x: f32, y: f32, sprite_id: u32) -> usize {
        let entity_type_enum = match entity_type {
            0 => EntityType::Player,
            1 => EntityType::Enemy,
            2 => EntityType::Platform,
            3 => EntityType::Projectile,
            4 => EntityType::Collectible,
            _ => EntityType::Collectible,
        };
        let entity = Entity::new(entity_type_enum, x, y, sprite_id);
        self.entities.push(entity);
        self.entities.len() - 1
    }

    pub fn handle_input(&mut self, up: bool, down: bool, left: bool, right: bool) {
        let speed = 2.0; // pixels per frame

        if up {
            self.move_sprite(0.0, -speed);
        }
        if down {
            self.move_sprite(0.0, speed);
        }
        if left {
            self.move_sprite(-speed, 0.0);
        }
        if right {
            self.move_sprite(speed, 0.0);
        }
    }

    fn get_palette_color(&self, index: u8) -> (u8, u8, u8) {
        MASTER_PALETTE[index as usize % 128]
    }

    fn render_color_test(&mut self) {
        // Display all 128 colors in a grid
        const COLORS_PER_ROW: usize = 16;
        const COLOR_SIZE: usize = 20; // 20x20 pixel squares

        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let pixel_index = (y * SCREEN_WIDTH + x) * 4;

                // Calculate which color square we're in
                let grid_x = x / COLOR_SIZE;
                let grid_y = y / COLOR_SIZE;

                // Calculate palette index
                let palette_index = (grid_y * COLORS_PER_ROW + grid_x) as u8;

                if palette_index < 128 && grid_x < COLORS_PER_ROW {
                    // Draw color square
                    let (r, g, b) = self.get_palette_color(palette_index);
                    self.screen_buffer[pixel_index] = r;
                    self.screen_buffer[pixel_index + 1] = g;
                    self.screen_buffer[pixel_index + 2] = b;
                    self.screen_buffer[pixel_index + 3] = 255;

                    // Add border between color squares
                    if x % COLOR_SIZE == 0 || y % COLOR_SIZE == 0 {
                        self.screen_buffer[pixel_index] = 128;     // Gray border
                        self.screen_buffer[pixel_index + 1] = 128;
                        self.screen_buffer[pixel_index + 2] = 128;
                        self.screen_buffer[pixel_index + 3] = 255;
                    }
                } else {
                    // Black background for areas without colors
                    self.screen_buffer[pixel_index] = 0;
                    self.screen_buffer[pixel_index + 1] = 0;
                    self.screen_buffer[pixel_index + 2] = 0;
                    self.screen_buffer[pixel_index + 3] = 255;
                }
            }
        }

        // Add text labels for color ranges
        self.render_color_labels();
    }

    fn render_color_labels(&mut self) {
        // Simple text rendering for color range labels
        // This is a basic implementation - would need proper font system for production

        const LABEL_Y: usize = 165; // Below the color grid
        const LABEL_HEIGHT: usize = 8;

        // Labels for each color family
        let labels = [
            (0, "GRAY"), (16, "RED"), (32, "ORG"), (48, "GRN"),
            (64, "CYN"), (80, "BLU"), (96, "PUR"), (112, "SKIN")
        ];

        for (start_color, label) in labels.iter() {
            let x_pos = (start_color * 20) + 2; // Position based on color grid

            // Simple 3x5 pixel font rendering (very basic)
            self.render_simple_text(x_pos, LABEL_Y, label);
        }

        // Add palette index numbers
        for color_idx in 0..128 {
            if color_idx % 16 == 0 { // Only show every 16th number to avoid clutter
                let x_pos = (color_idx % 16) * 20 + 2;
                let y_pos = (color_idx / 16) * 20 + 2;

                // Render color index number (simplified)
                let index_str = format!("{}", color_idx);
                self.render_simple_text(x_pos, y_pos, &index_str);
            }
        }
    }

    fn render_simple_text(&mut self, x: usize, y: usize, text: &str) {
        // Very basic text rendering - just white pixels in a simple pattern
        for (char_idx, ch) in text.chars().enumerate() {
            let char_x = x + char_idx * 4; // 4 pixels wide per character

            if char_x + 3 >= SCREEN_WIDTH || y + 4 >= SCREEN_HEIGHT {
                continue;
            }

            // Simple character patterns (very minimal)
            let pattern = match ch {
                'G' => [0b111, 0b100, 0b110, 0b100, 0b111],
                'R' => [0b111, 0b101, 0b111, 0b110, 0b101],
                'A' => [0b111, 0b101, 0b111, 0b101, 0b101],
                'Y' => [0b101, 0b101, 0b111, 0b010, 0b010],
                'E' => [0b111, 0b100, 0b111, 0b100, 0b111],
                'D' => [0b110, 0b101, 0b101, 0b101, 0b110],
                'O' => [0b111, 0b101, 0b101, 0b101, 0b111],
                'N' => [0b111, 0b101, 0b101, 0b101, 0b101],
                'C' => [0b111, 0b100, 0b100, 0b100, 0b111],
                'B' => [0b111, 0b101, 0b110, 0b101, 0b111],
                'L' => [0b100, 0b100, 0b100, 0b100, 0b111],
                'U' => [0b101, 0b101, 0b101, 0b101, 0b111],
                'P' => [0b111, 0b101, 0b111, 0b100, 0b100],
                'I' => [0b111, 0b010, 0b010, 0b010, 0b111],
                'S' => [0b111, 0b100, 0b111, 0b001, 0b111],
                'K' => [0b101, 0b110, 0b100, 0b110, 0b101],
                'T' => [0b111, 0b010, 0b010, 0b010, 0b010],
                'M' => [0b101, 0b111, 0b111, 0b101, 0b101],
                '0'..='9' => {
                    let digit = ch as u8 - b'0';
                    match digit {
                        0 => [0b111, 0b101, 0b101, 0b101, 0b111],
                        1 => [0b010, 0b110, 0b010, 0b010, 0b111],
                        2 => [0b111, 0b001, 0b111, 0b100, 0b111],
                        3 => [0b111, 0b001, 0b111, 0b001, 0b111],
                        4 => [0b101, 0b101, 0b111, 0b001, 0b001],
                        5 => [0b111, 0b100, 0b111, 0b001, 0b111],
                        6 => [0b111, 0b100, 0b111, 0b101, 0b111],
                        7 => [0b111, 0b001, 0b001, 0b001, 0b001],
                        8 => [0b111, 0b101, 0b111, 0b101, 0b111],
                        9 => [0b111, 0b101, 0b111, 0b001, 0b111],
                        _ => [0b000, 0b000, 0b000, 0b000, 0b000],
                    }
                }
                _ => [0b000, 0b000, 0b000, 0b000, 0b000], // Space or unknown
            };

            // Draw the character pattern
            for (row, pattern_row) in pattern.iter().enumerate() {
                for col in 0..3 {
                    if pattern_row & (1 << (2 - col)) != 0 {
                        let px = char_x + col;
                        let py = y + row;
                        if px < SCREEN_WIDTH && py < SCREEN_HEIGHT {
                            let pixel_idx = (py * SCREEN_WIDTH + px) * 4;
                            self.screen_buffer[pixel_idx] = 255;     // White text
                            self.screen_buffer[pixel_idx + 1] = 255;
                            self.screen_buffer[pixel_idx + 2] = 255;
                            self.screen_buffer[pixel_idx + 3] = 255;
                        }
                    }
                }
            }
        }
    }

    pub fn toggle_color_test_mode(&mut self) {
        self.color_test_mode = !self.color_test_mode;
    }

    // New text rendering system
    fn render_char_8x8(&mut self, ch: char, x: usize, y: usize, color_index: u8) {
        if x + 8 > SCREEN_WIDTH || y + 8 > SCREEN_HEIGHT {
            return; // Out of bounds
        }

        let ascii_code = ch as u8;
        if ascii_code < 32 || ascii_code > 126 {
            return; // Character not in font
        }

        let font_index = (ascii_code - 32) as usize;
        let char_data = FONT_8X8[font_index];

        let (r, g, b) = self.get_palette_color(color_index);

        for row in 0..8 {
            for col in 0..8 {
                if char_data[row] & (0x01 << col) != 0 {  // Changed from 0x80 >> col to 0x01 << col
                    let pixel_x = x + col;
                    let pixel_y = y + row;
                    let pixel_index = (pixel_y * SCREEN_WIDTH + pixel_x) * 4;

                    self.screen_buffer[pixel_index] = r;
                    self.screen_buffer[pixel_index + 1] = g;
                    self.screen_buffer[pixel_index + 2] = b;
                    self.screen_buffer[pixel_index + 3] = 255;
                }
            }
        }
    }

    fn render_text(&mut self, text: &str, x: usize, y: usize, color_index: u8) {
        for (i, ch) in text.chars().enumerate() {
            let char_x = x + (i * 8);
            if char_x + 8 > SCREEN_WIDTH {
                break; // Stop if we'd go off screen
            }
            self.render_char_8x8(ch, char_x, y, color_index);
        }
    }

    fn render_text_centered(&mut self, text: &str, y: usize, color_index: u8) {
        let text_width = text.len() * 8;
        if text_width > SCREEN_WIDTH {
            return; // Text too long
        }
        let x = (SCREEN_WIDTH - text_width) / 2;
        self.render_text(text, x, y, color_index);
    }
}