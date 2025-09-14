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

#[derive(Clone, Copy, PartialEq)]
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
            EntityType::Player => (24.0, 20.0),  // Hambert size
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

                // Update physics once per frame
                self.update_physics();

                return true; // Frame complete
            }
        }

        false
    }

    fn update_physics(&mut self) {
        const GRAVITY: f32 = 0.4;  // Pixels per frame squared
        const MAX_FALL_SPEED: f32 = 8.0;  // Terminal velocity
        const GROUND_LEVEL: f32 = 200.0;  // Simple ground level for now

        for entity in &mut self.entities {
            if !entity.active {
                continue;
            }

            match entity.entity_type {
                EntityType::Player => {
                    // Apply gravity
                    if !entity.on_ground {
                        entity.vel_y += GRAVITY;
                        if entity.vel_y > MAX_FALL_SPEED {
                            entity.vel_y = MAX_FALL_SPEED;
                        }
                    }

                    // Apply velocity
                    entity.x += entity.vel_x;
                    entity.y += entity.vel_y;

                    // Ground collision (simple for now)
                    if entity.y + entity.height >= GROUND_LEVEL {
                        entity.y = GROUND_LEVEL - entity.height;
                        entity.vel_y = 0.0;
                        entity.on_ground = true;
                    } else {
                        entity.on_ground = false;
                    }

                    // Platform collision will be handled after the main loop

                    // Apply friction when on ground
                    if entity.on_ground {
                        entity.vel_x *= 0.85; // Friction
                    } else {
                        entity.vel_x *= 0.98; // Air resistance
                    }

                    // Keep within world bounds
                    entity.x = entity.x.max(0.0).min(self.world_width - entity.width);

                    // Update sprite position for backward compatibility
                    if entity.entity_type == EntityType::Player {
                        self.sprite_x = entity.x;
                        self.sprite_y = entity.y;
                        // Camera will be updated after physics loop
                    }
                }
                _ => {
                    // Other entity types can have different physics later
                }
            }
        }

        // Handle platform collisions after main physics loop
        self.check_platform_collisions(self.player_id);

        // Update camera to follow player after physics
        if self.player_id < self.entities.len() {
            self.update_camera_follow_player();
        }
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
        // Render the layered background system (back to front)
        self.render_sky_gradient();
        self.render_clouds();          // Clouds in the sky
        self.render_mountains();       // Mountains behind trees
        self.render_background_trees(); // Trees between mountains and foreground
        self.render_ground_terrain();  // Ground in front of everything else

        // Render all entities (player, platforms, enemies)
        self.render_entities();

        // Add demo text
        self.render_text_centered("Hi, Chris. This is how text will look ;P", 40, 15); // White text
        self.render_text("Use WASD/Arrow Keys to move", 10, 200, 80); // Blue text
        self.render_text("Press Enter for color test", 10, 210, 48); // Green text
    }

    fn render_hambert_sprite(&mut self, x: usize, y: usize) {
        // 24x20 Hambert sprite data from hambertBoy.js
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

        // Hambert color palette mapped to our system
        let hambert_colors = [
            0,   // 0 - transparent -> black in our palette
            10,  // 1 - mid-light gray fur (lighter than original)
            0,   // 2 - #000000 eye outline -> black
            15,  // 3 - #ffffff eye white -> white from our palette
            0,   // 4 - #000000 black nose -> black
            16,  // 5 - #cc0000 red boots -> red from our palette
            120, // 6 - #ff6666 pink tongue -> pink from our palette
            32,  // 7 - #654321 brown boot tops -> brown from our palette
        ];

        for (sprite_y, row) in pixel_data.iter().enumerate() {
            for (sprite_x, &pixel) in row.iter().enumerate() {
                let screen_x = x + sprite_x;
                let screen_y = y + sprite_y;

                // Bounds check
                if screen_x >= SCREEN_WIDTH || screen_y >= SCREEN_HEIGHT {
                    continue;
                }

                // Skip transparent pixels (0)
                if pixel == 0 {
                    continue;
                }

                // Get color from our palette
                let palette_index = hambert_colors.get(pixel as usize).copied().unwrap_or(0);
                let (r, g, b) = self.get_palette_color(palette_index);

                let pixel_index = (screen_y * SCREEN_WIDTH + screen_x) * 4;
                self.screen_buffer[pixel_index] = r;
                self.screen_buffer[pixel_index + 1] = g;
                self.screen_buffer[pixel_index + 2] = b;
                self.screen_buffer[pixel_index + 3] = 255;
            }
        }
    }

    fn render_hambert_walk_sprite(&mut self, x: usize, y: usize) {
        // 24x20 Hambert walking sprite data from hambertBoy.js
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
            [0,0,0,0,0,7,7,7,7,0,0,0,0,7,7,7,7,7,0,0,0,0,0,0], // left boot lower
            [0,0,0,0,0,5,5,5,5,5,0,0,5,5,5,5,5,5,0,0,0,0,0,0], // walking animation
            [0,0,0,0,0,5,5,5,5,5,0,0,5,5,5,5,5,5,0,0,0,0,0,0], // boots shifted
        ];

        // Same color palette as idle Hambert
        let hambert_colors = [
            0,   // 0 - transparent
            10,  // 1 - mid-light gray fur (lighter than original)
            0,   // 2 - eye outline black
            15,  // 3 - eye white
            0,   // 4 - black nose
            16,  // 5 - red boots
            120, // 6 - pink tongue
            32,  // 7 - brown boot tops
        ];

        for (sprite_y, row) in pixel_data.iter().enumerate() {
            for (sprite_x, &pixel) in row.iter().enumerate() {
                let screen_x = x + sprite_x;
                let screen_y = y + sprite_y;

                if screen_x >= SCREEN_WIDTH || screen_y >= SCREEN_HEIGHT {
                    continue;
                }

                if pixel == 0 {
                    continue;
                }

                let palette_index = hambert_colors.get(pixel as usize).copied().unwrap_or(0);
                let (r, g, b) = self.get_palette_color(palette_index);

                let pixel_index = (screen_y * SCREEN_WIDTH + screen_x) * 4;
                self.screen_buffer[pixel_index] = r;
                self.screen_buffer[pixel_index + 1] = g;
                self.screen_buffer[pixel_index + 2] = b;
                self.screen_buffer[pixel_index + 3] = 255;
            }
        }
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
                        self.render_hambert_sprite(screen_x as usize, screen_y as usize);
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
        for dy in 0..height {
            for dx in 0..width {
                let screen_x = x + dx;
                let screen_y = y + dy;

                if screen_x >= 0 && screen_x < SCREEN_WIDTH as i32 &&
                   screen_y >= 0 && screen_y < SCREEN_HEIGHT as i32 {
                    let pixel_index = ((screen_y as usize * SCREEN_WIDTH) + screen_x as usize) * 4;
                    if pixel_index + 3 >= self.screen_buffer.len() {
                        continue;
                    }

                    // Create grass-topped platform with different layers
                    let platform_color = if dy == 0 {
                        // Top row: bright green grass
                        50u8
                    } else if dy <= 2 {
                        // Next rows: darker green grass/dirt mix
                        49u8
                    } else if dy <= 4 {
                        // Brown dirt layer
                        33u8
                    } else if dy <= 6 {
                        // Darker brown earth
                        34u8
                    } else {
                        // Deep dark brown/stone
                        35u8
                    };

                    // Add some texture variation
                    let texture_variation = ((screen_x + screen_y * 3) % 4) as u8;
                    let final_color = if texture_variation == 0 && dy > 0 {
                        // Slightly darker for texture
                        (platform_color + 1).min(127)
                    } else {
                        platform_color
                    };

                    let (r, g, b) = self.get_palette_color(final_color);
                    self.screen_buffer[pixel_index] = r;
                    self.screen_buffer[pixel_index + 1] = g;
                    self.screen_buffer[pixel_index + 2] = b;
                    self.screen_buffer[pixel_index + 3] = 255;
                }
            }
        }

        // Add grass tufts on top of platform
        self.render_grass_tufts(x, y, width);
    }

    fn render_grass_tufts(&mut self, platform_x: i32, platform_y: i32, platform_width: i32) {
        // Add small grass tufts on top of platforms
        let grass_color = self.get_palette_color(48u8); // Bright green

        for tuft_x in (0..platform_width).step_by(8) {
            let screen_x = platform_x + tuft_x;
            let screen_y = platform_y - 1; // Just above the platform

            if screen_x >= 0 && screen_x < SCREEN_WIDTH as i32 &&
               screen_y >= 0 && screen_y < SCREEN_HEIGHT as i32 {
                let pixel_index = ((screen_y as usize * SCREEN_WIDTH) + screen_x as usize) * 4;

                // Simple grass tuft pattern
                let tuft_pattern = [
                    (0, 0), (1, 0), (2, 0),    // Base of tuft
                    (1, -1),                   // Tip of tuft
                ];

                for (dx, dy) in tuft_pattern.iter() {
                    let grass_x = screen_x + dx;
                    let grass_y = screen_y + dy;

                    if grass_x >= 0 && grass_x < SCREEN_WIDTH as i32 &&
                       grass_y >= 0 && grass_y < SCREEN_HEIGHT as i32 {
                        let grass_pixel = ((grass_y as usize * SCREEN_WIDTH) + grass_x as usize) * 4;
                        if grass_pixel + 3 < self.screen_buffer.len() {
                            self.screen_buffer[grass_pixel] = grass_color.0;
                            self.screen_buffer[grass_pixel + 1] = grass_color.1;
                            self.screen_buffer[grass_pixel + 2] = grass_color.2;
                            self.screen_buffer[grass_pixel + 3] = 255;
                        }
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

    // Background rendering system for hambertBoy-style environments
    fn render_sky_gradient(&mut self) {
        // Create a vertical gradient from light blue (top) to lighter blue/white (bottom)
        // Using palette colors: light blue to cyan to white
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

            let (r, g, b) = self.get_palette_color(palette_index);

            // Fill the entire width with this color
            for x in 0..SCREEN_WIDTH {
                let pixel_index = (y * SCREEN_WIDTH + x) * 4;
                self.screen_buffer[pixel_index] = r;
                self.screen_buffer[pixel_index + 1] = g;
                self.screen_buffer[pixel_index + 2] = b;
                self.screen_buffer[pixel_index + 3] = 255;
            }
        }
    }

    fn render_mountains(&mut self) {
        // Parallax mountain silhouettes in the background
        // Mountains scroll slower than the camera for depth effect
        let mountain_parallax_factor = 0.3; // Mountains move 30% of camera speed
        let mountain_offset = self.camera_x * mountain_parallax_factor;

        // Render mountain layers (back to front)
        self.render_mountain_layer(mountain_offset * 0.5, 100, 96u8);  // Far mountains (purple)
        self.render_mountain_layer(mountain_offset * 0.7, 120, 80u8);  // Mid mountains (darker blue)
        self.render_mountain_layer(mountain_offset, 140, 48u8);        // Near mountains (dark green)
    }

    fn render_mountain_layer(&mut self, offset: f32, base_height: usize, color_index: u8) {
        let (r, g, b) = self.get_palette_color(color_index);

        // Create mountain silhouette using a simple sin wave pattern
        for x in 0..SCREEN_WIDTH {
            let world_x = x as f32 + offset;

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
                    self.screen_buffer[pixel_index] = ((r as u16 + existing_r as u16) / 2) as u8;
                    self.screen_buffer[pixel_index + 1] = ((g as u16 + existing_g as u16) / 2) as u8;
                    self.screen_buffer[pixel_index + 2] = ((b as u16 + existing_b as u16) / 2) as u8;
                    self.screen_buffer[pixel_index + 3] = 255;
                }
            }
        }
    }

    fn render_ground_terrain(&mut self) {
        // Render ground level terrain that scrolls with camera
        let ground_level = 200; // Base ground level

        for x in 0..SCREEN_WIDTH {
            let world_x = x as f32 + self.camera_x;

            // Create slight terrain variation
            let terrain_height = ((world_x * 0.02).sin() * 5.0) as i32;
            let actual_ground = (ground_level + terrain_height) as usize;

            // Render ground from terrain level to bottom
            for y in actual_ground..SCREEN_HEIGHT {
                if y < SCREEN_HEIGHT {
                    let pixel_index = (y * SCREEN_WIDTH + x) * 4;

                    // Ground color based on depth
                    let depth = y - actual_ground;
                    let ground_color = if depth < 5 {
                        49u8  // Bright green grass
                    } else if depth < 15 {
                        33u8  // Brown dirt
                    } else {
                        34u8  // Darker brown deeper underground
                    };

                    let (r, g, b) = self.get_palette_color(ground_color);
                    self.screen_buffer[pixel_index] = r;
                    self.screen_buffer[pixel_index + 1] = g;
                    self.screen_buffer[pixel_index + 2] = b;
                    self.screen_buffer[pixel_index + 3] = 255;
                }
            }
        }
    }

    fn render_clouds(&mut self) {
        // Render puffy white clouds with parallax scrolling
        let cloud_parallax = 0.2; // Clouds move slower than camera
        let cloud_offset = self.camera_x * cloud_parallax;

        // Render multiple cloud layers
        self.render_cloud_layer(cloud_offset, 30, 87u8);       // High clouds (light blue-white)
        self.render_cloud_layer(cloud_offset * 1.3, 50, 255u8); // Mid clouds (white)
        self.render_cloud_layer(cloud_offset * 0.7, 20, 86u8);  // Lower clouds (light blue)
    }

    fn render_cloud_layer(&mut self, offset: f32, base_y: usize, color_index: u8) {
        let (r, g, b) = self.get_palette_color(color_index);

        // Generate cloud shapes using noise-like patterns
        for cloud_center in (0..800).step_by(120) {
            let cloud_x = (cloud_center as f32 + offset) % (SCREEN_WIDTH as f32 + 200.0) - 100.0;
            let cloud_y = base_y as f32 + ((cloud_center as f32 * 0.01).sin() * 10.0);

            self.render_single_cloud(cloud_x as i32, cloud_y as i32, color_index);
        }
    }

    fn render_single_cloud(&mut self, center_x: i32, center_y: i32, color_index: u8) {
        let (r, g, b) = self.get_palette_color(color_index);

        // Cloud shape using overlapping circles
        let cloud_parts = [
            (0, 0, 16),    // Center
            (-12, -4, 12), // Left
            (12, -4, 12),  // Right
            (-8, 4, 10),   // Bottom left
            (8, 4, 10),    // Bottom right
            (0, -8, 8),    // Top
        ];

        for (dx, dy, radius) in cloud_parts.iter() {
            let part_x = center_x + dx;
            let part_y = center_y + dy;

            for y in (part_y - radius)..(part_y + radius) {
                for x in (part_x - radius)..(part_x + radius) {
                    if x >= 0 && x < SCREEN_WIDTH as i32 && y >= 0 && y < SCREEN_HEIGHT as i32 {
                        let dist_sq = (x - part_x) * (x - part_x) + (y - part_y) * (y - part_y);
                        if dist_sq <= (radius * radius) {
                            let pixel_index = ((y as usize * SCREEN_WIDTH) + x as usize) * 4;
                            if pixel_index + 3 < self.screen_buffer.len() {
                                // Soft alpha blending with existing sky
                                let existing_r = self.screen_buffer[pixel_index];
                                let existing_g = self.screen_buffer[pixel_index + 1];
                                let existing_b = self.screen_buffer[pixel_index + 2];

                                let alpha = 0.7; // Cloud opacity
                                self.screen_buffer[pixel_index] = ((r as f32 * alpha + existing_r as f32 * (1.0 - alpha)) as u8);
                                self.screen_buffer[pixel_index + 1] = ((g as f32 * alpha + existing_g as f32 * (1.0 - alpha)) as u8);
                                self.screen_buffer[pixel_index + 2] = ((b as f32 * alpha + existing_b as f32 * (1.0 - alpha)) as u8);
                                self.screen_buffer[pixel_index + 3] = 255;
                            }
                        }
                    }
                }
            }
        }
    }

    fn render_background_trees(&mut self) {
        // Render stylized background trees with parallax
        let tree_parallax = 0.4; // Trees move slower than foreground
        let tree_offset = self.camera_x * tree_parallax;

        // Place trees at regular intervals
        for tree_pos in (0..1200).step_by(80) {
            let tree_x = (tree_pos as f32 + tree_offset) % (SCREEN_WIDTH as f32 + 100.0) - 50.0;
            let ground_y = 200.0 + ((tree_x * 0.02).sin() * 5.0); // Follow ground contour

            self.render_single_tree(tree_x as i32, ground_y as i32);
        }
    }

    fn render_single_tree(&mut self, base_x: i32, base_y: i32) {
        // Simple tree silhouette - trunk and crown
        let trunk_width = 6;
        let trunk_height = 40;
        let crown_radius = 20;

        // Render trunk
        let trunk_color = self.get_palette_color(34u8); // Dark brown
        for y in (base_y - trunk_height)..base_y {
            for x in (base_x - trunk_width/2)..(base_x + trunk_width/2) {
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
        let crown_color = self.get_palette_color(48u8); // Dark green
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
        if self.player_id >= self.entities.len() {
            return;
        }

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

    fn check_platform_collisions(&mut self, entity_index: usize) {
        let entity_bounds = {
            let entity = &self.entities[entity_index];
            (entity.x, entity.y, entity.x + entity.width, entity.y + entity.height, entity.vel_y)
        };

        let (ex1, ey1, ex2, ey2, vel_y) = entity_bounds;

        // Check collision with all platform entities
        for i in 0..self.entities.len() {
            if i == entity_index {
                continue;
            }

            let platform = &self.entities[i];
            if platform.entity_type != EntityType::Platform || !platform.active {
                continue;
            }

            let px1 = platform.x;
            let py1 = platform.y;
            let px2 = platform.x + platform.width;
            let _py2 = platform.y + platform.height;

            // Check if entity is falling down onto platform
            if vel_y > 0.0 && ex1 < px2 && ex2 > px1 && ey2 >= py1 && ey1 < py1 {
                // Landing on top of platform
                let entity = &mut self.entities[entity_index];
                entity.y = py1 - entity.height;
                entity.vel_y = 0.0;
                entity.on_ground = true;
                break;
            }
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

    pub fn handle_input(&mut self, up: bool, _down: bool, left: bool, right: bool) {
        if self.player_id >= self.entities.len() {
            return;
        }

        let player = &mut self.entities[self.player_id];
        const MOVE_SPEED: f32 = 1.5;  // Horizontal acceleration
        const JUMP_STRENGTH: f32 = -7.0;  // Negative because Y increases downward

        // Horizontal movement
        if left {
            player.vel_x -= MOVE_SPEED;
            if player.vel_x < -4.0 {  // Max speed left
                player.vel_x = -4.0;
            }
        }
        if right {
            player.vel_x += MOVE_SPEED;
            if player.vel_x > 4.0 {  // Max speed right
                player.vel_x = 4.0;
            }
        }

        // Jumping - only when on ground
        if up && player.on_ground {
            player.vel_y = JUMP_STRENGTH;
            player.on_ground = false;
        }

        // Down can be used later for crouching or dropping through platforms
        // For now, let's ignore it or use it for debug
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



