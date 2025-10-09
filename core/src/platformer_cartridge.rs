// Tileset data - 256 tiles from test_tileset.png
include!("../../TILESETS/test_tileset_data.rs");

// Real Hambert sprite data - extracted from actual PNG files
// Generated from hambert_idle.png, hambert_walk1.png, hambert_walk2.png

const HAMBERT_IDLE_SPRITE: [[u8; 16]; 16] = [
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 8, 10, 10, 8, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 8, 10, 10, 10, 10, 8, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 8, 8, 8, 10, 8, 8, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 8, 10, 10, 10, 10, 10, 10, 8, 0, 0, 0],
    [0, 0, 0, 0, 8, 8, 10, 8, 8, 10, 8, 8, 10, 8, 0, 0],
    [0, 0, 0, 8, 10, 8, 10, 8, 8, 10, 8, 8, 10, 8, 10, 0],
    [0, 0, 0, 8, 10, 10, 10, 15, 1, 8, 15, 1, 10, 8, 10, 0],
    [0, 0, 0, 8, 8, 10, 10, 10, 10, 8, 10, 10, 10, 8, 0, 0],
    [0, 0, 0, 0, 0, 8, 8, 10, 10, 0, 10, 10, 8, 0, 0, 0],
    [0, 0, 0, 0, 0, 8, 10, 10, 10, 8, 8, 10, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 8, 8, 10, 10, 27, 10, 10, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 45, 45, 8, 10, 10, 10, 8, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 22, 22, 22, 8, 8, 8, 45, 45, 0, 0, 0],
    [0, 0, 0, 0, 45, 22, 22, 22, 22, 22, 0, 45, 45, 45, 0, 0],
    [0, 0, 0, 0, 22, 45, 45, 22, 22, 22, 22, 0, 45, 22, 45, 0],
];

const HAMBERT_WALK1_SPRITE: [[u8; 16]; 16] = [
    [0,0,0,0,0,0,1,8,10,10,8,1,0,0,0,0],
    [0,0,0,0,0,1,8,10,10,10,10,8,1,0,0,0],
    [0,0,0,0,0,1,8,8,8,10,8,8,1,0,0,0],
    [0,0,0,0,1,8,10,10,10,10,10,10,8,1,0,0],
    [0,0,0,1,8,8,10,8,8,10,8,8,10,8,1,0],
    [0,0,0,8,10,8,10,8,8,10,8,8,10,8,10,0],
    [0,0,1,8,10,10,10,15,1,8,15,1,10,8,10,0],
    [0,0,1,8,8,10,10,10,10,8,10,10,10,8,1,0],
    [0,0,0,0,1,8,8,10,10,1,10,10,8,0,0,0],
    [0,0,0,0,1,8,10,10,10,8,8,10,1,0,0,0],
    [0,0,0,0,1,8,8,10,10,106,10,10,1,0,0,0],
    [0,0,0,0,1,45,45,8,10,10,10,8,1,45,45,0],
    [0,0,0,0,1,22,22,22,8,8,8,45,45,45,45,0],
    [0,0,0,1,45,22,22,22,22,22,1,45,45,45,0,0],
    [0,0,0,1,22,45,45,22,22,22,22,1,45,0,0,0],
    [0,0,0,0,0,45,22,22,22,22,22,0,0,0,0,0],
];

const HAMBERT_WALK2_SPRITE: [[u8; 16]; 16] = [
    [0,0,0,0,0,0,1,8,10,10,8,1,0,0,0,0],
    [0,0,0,0,0,1,8,10,10,10,10,8,1,0,0,0],
    [0,0,0,0,0,1,8,8,8,10,8,8,1,0,0,0],
    [0,0,0,0,1,8,10,10,10,10,10,10,8,1,0,0],
    [0,0,0,1,8,8,10,8,8,10,8,8,10,8,1,0],
    [0,0,0,8,10,8,10,8,8,10,8,8,10,8,10,0],
    [0,0,1,8,10,10,10,15,1,8,15,1,10,8,10,0],
    [0,0,1,8,8,10,10,10,10,8,10,10,10,8,1,0],
    [0,0,0,0,1,8,8,10,10,1,10,10,8,0,0,0],
    [0,0,0,0,1,8,10,10,10,8,8,10,1,0,0,0],
    [0,0,0,0,1,8,8,10,10,106,22,22,1,0,0,0],
    [0,0,0,0,1,45,45,22,22,22,22,22,1,0,0,0],
    [0,0,0,0,1,22,22,22,22,22,22,1,45,0,0,0],
    [0,0,0,1,45,22,22,22,22,45,1,45,45,0,0,0],
    [0,0,0,1,22,22,22,22,0,0,45,45,45,22,0,0],
    [0,0,0,0,45,45,45,0,0,0,45,22,22,22,0,0],
];

// Glowing hexagon projectile - 6×6 small glowing hexagon (255 = transparent)
const HEXAGON_PROJECTILE_SPRITE: [[u8; 6]; 6] = [
    [255,20,21,21,20,255],
    [20,21,22,22,21,20],
    [21,22,23,23,22,21],
    [21,22,23,23,22,21],
    [20,21,22,22,21,20],
    [255,20,21,21,20,255],
];

// Hexagnome sprite - 26×32 stone gnome enemy (255 = transparent)
const HEXAGNOME_SPRITE: [[u8; 26]; 32] = [
    [255,255,255,255,255,255,255,255,1,3,3,4,4,3,3,255,255,255,255,255,255,255,255,255,255,255],
    [255,255,255,255,255,255,255,3,255,3,3,4,4,3,3,2,1,255,255,255,255,255,255,255,255,255],
    [255,255,255,255,255,255,255,1,6,5,2,3,3,3,2,2,2,4,255,255,255,255,255,255,255,255],
    [255,255,255,255,255,255,255,8,13,7,2,2,2,2,2,4,4,4,3,255,255,255,255,255,255,255],
    [255,255,255,255,255,255,255,13,11,11,4,2,2,3,4,5,4,4,3,255,255,255,255,255,255,255],
    [255,255,255,255,255,255,11,10,10,9,3,255,2,3,2,2,2,4,5,4,255,255,255,255,255,255],
    [255,255,255,255,255,9,12,9,3,1,1,1,1,2,1,2,5,4,5,5,3,255,255,255,255,255],
    [255,255,255,255,255,255,4,1,1,1,1,1,2,2,3,5,6,5,4,5,4,4,255,255,255,255],
    [255,255,255,255,255,255,255,255,255,255,1,1,2,3,5,5,6,5,4,5,4,3,255,255,255,255],
    [255,255,255,255,255,255,255,255,255,255,255,1,2,4,5,5,6,4,4,6,5,2,2,255,255,255],
    [255,255,255,255,255,255,255,255,255,3,1,1,3,4,5,6,6,4,4,5,6,2,2,255,255,255],
    [255,255,255,255,255,255,255,255,3,1,1,1,3,5,5,6,5,3,4,5,5,3,3,255,255,255],
    [255,255,255,255,255,255,3,3,3,1,255,1,3,5,5,6,5,3,4,4,5,3,3,255,255,255],
    [255,3,3,3,3,3,3,3,3,1,2,3,3,4,5,6,3,3,4,4,6,3,2,255,255,255],
    [255,3,13,13,12,3,3,3,2,3,3,3,4,4,4,4,3,3,4,5,4,1,255,1,255,255],
    [255,255,3,13,12,3,3,3,3,3,3,3,4,3,3,4,4,4,4,5,5,3,2,1,255,255],
    [255,255,3,3,3,255,255,3,3,3,3,4,3,2,3,5,4,4,4,5,4,3,5,3,255,255],
    [255,255,255,255,255,255,3,3,3,3,2,2,1,1,4,5,4,4,4,4,4,4,2,3,3,255],
    [255,255,255,255,255,2,3,3,2,2,1,1,1,1,2,4,4,4,4,4,4,4,2,2,3,255],
    [255,255,255,255,255,5,2,2,1,255,1,2,3,3,3,3,4,4,4,4,4,3,2,2,3,255],
    [255,255,255,255,3,12,12,8,1,2,2,3,4,4,3,3,4,4,4,4,4,3,2,2,3,255],
    [255,255,255,255,3,13,13,7,3,3,3,4,4,4,3,3,3,4,4,4,3,3,2,3,2,255],
    [255,255,255,255,3,3,3,3,3,3,3,3,4,5,4,4,4,4,4,3,3,2,2,3,3,255],
    [255,255,255,255,255,255,255,255,3,3,3,2,3,4,5,5,4,4,3,3,3,2,1,3,255,255],
    [255,255,255,255,255,255,255,255,3,3,4,3,2,2,3,4,4,3,3,3,3,2,1,3,255,255],
    [255,255,255,255,255,255,255,4,3,3,4,5,4,3,3,3,4,3,3,3,2,2,3,3,255,255],
    [255,255,255,255,255,255,255,3,3,3,4,4,4,4,4,3,3,3,3,3,2,2,3,3,255,255],
    [255,255,255,255,255,255,255,3,3,2,3,4,4,4,4,3,3,3,3,3,2,255,255,255,255,255],
    [255,255,255,255,255,255,255,255,255,255,3,3,3,3,3,3,3,3,3,3,255,255,255,255,255,255],
    [255,255,255,255,255,255,255,255,255,3,3,3,3,3,2,3,3,3,3,3,255,255,255,255,255,255],
    [255,255,255,255,255,255,255,255,3,3,3,3,3,2,3,3,3,3,3,3,3,255,255,255,255,255],
    [255,255,255,255,255,255,3,3,3,3,3,3,2,3,3,3,3,3,3,3,3,255,255,255,255,255],
];

#[derive(Debug, Clone, PartialEq)]
pub enum TileCollisionType {
    None,
    Solid,
    OneWayPlatform,
    Pitfall,        // Deadly - kills player
    Passage,        // Can fall through - leads to vertical areas
    Water,          // Swimming area
    SwimThrough,    // Can only pass through when swimming
}

#[derive(Debug, Clone, PartialEq)]
pub enum PlayerState {
    Walking,
    Jumping,
    Falling,
    Swimming,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GameState {
    TitleScreen,
    Cutscene,
    Playing,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CutsceneType {
    Intro,
    Interlude1,
    Interlude2,
    Interlude3,
    Interlude4,
    Ending,
}

#[derive(Debug, Clone)]
pub struct CutsceneScreen {
    pub image: [[u8; 64]; 64],
    pub text_lines: Vec<String>,
    pub sound_id: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct Cutscene {
    pub cutscene_type: CutsceneType,
    pub screens: Vec<CutsceneScreen>,
    pub current_screen: usize,
    pub text_scroll_offset: f32,
    pub text_scroll_speed: f32,
    pub text_char_index: usize,  // Current character being displayed
    pub text_char_timer: f32,    // Timer for character reveal
    pub chars_per_frame: f32,    // Speed of typing (0.5 = 2 frames per char, 1.0 = 1 char per frame)
}

#[derive(Debug, Clone)]
pub struct Hexagnome {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub patrol_left: f32,
    pub patrol_right: f32,
    pub facing_right: bool,
    pub shoot_timer: f32,
}

#[derive(Debug, Clone)]
pub struct Projectile {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub active: bool,
}

pub struct PlatformerCartridge {
    game_state: GameState,
    title_blink_timer: f32, // Timer for blinking "PRESS START"
    current_cutscene: Option<Cutscene>,
    current_level: u32,  // Track current level (0-4)
    previous_input: u8,  // Track previous frame's input for button press detection
    camera_x: f32,
    camera_y: f32,
    player_x: f32,
    player_y: f32,
    player_vx: f32,
    player_vy: f32,
    on_ground: bool,
    player_state: PlayerState,
    animation_frame: u32,
    animation_timer: f32,
    facing_right: bool, // true = facing right, false = facing left
    level_width: f32,
    level_height: f32,
    // Jump mechanics
    jump_button_held: bool, // Track if jump button is currently held
    jump_hold_time: f32, // How long jump has been held (in frames)
    jumping: bool, // True if currently in a jump
    // Sound effects
    pending_sounds: Vec<u32>,
    // Large tile map - 200x15 tiles (3200x240 pixels with 16x16 tiles)
    tiles: [[u8; 200]; 15],
    // Enemies/NPCs
    hexagnomes: Vec<Hexagnome>,
    projectiles: Vec<Projectile>,
    // Background music
    music_enabled: bool,
    music_step: usize,
    music_timer: f32,
    music_tempo: f32, // Frames per step
}

impl PlatformerCartridge {
    pub fn new() -> Self {
        // Level imported from Tiled (test_level.tmx)
        // Tile types: 0=air, 1=solid block, 2=platform (oneway), 3=pitfall (deadly), 4=passage (leads down), 5=water, 6=swim-through
        let tiles: [[u8; 200]; 15] = [
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 3, 3, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 3, 3, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        ];

        Self {
            game_state: GameState::TitleScreen,
            title_blink_timer: 0.0,
            current_cutscene: None,
            current_level: 0,
            previous_input: 0,
            camera_x: 0.0,
            camera_y: 0.0,
            player_x: 50.0,
            player_y: 150.0,
            player_vx: 0.0,
            player_vy: 0.0,
            on_ground: false,
            player_state: PlayerState::Falling,
            animation_frame: 0,
            animation_timer: 0.0,
            facing_right: true, // Start facing right
            level_width: 3200.0, // 200 tiles * 16 pixels
            level_height: 240.0, // 15 tiles * 16 pixels
            jump_button_held: false,
            jump_hold_time: 0.0,
            jumping: false,
            pending_sounds: Vec::new(),
            tiles,
            hexagnomes: vec![
                Hexagnome {
                    x: 100.0, y: 192.0, vx: 1.0,
                    patrol_left: 50.0, patrol_right: 150.0,
                    facing_right: true, shoot_timer: 0.0
                },
                Hexagnome {
                    x: 400.0, y: 192.0, vx: 1.0,
                    patrol_left: 350.0, patrol_right: 450.0,
                    facing_right: true, shoot_timer: 0.0
                },
                Hexagnome {
                    x: 800.0, y: 192.0, vx: 1.0,
                    patrol_left: 750.0, patrol_right: 850.0,
                    facing_right: true, shoot_timer: 0.0
                },
                Hexagnome {
                    x: 1200.0, y: 192.0, vx: 1.0,
                    patrol_left: 1150.0, patrol_right: 1250.0,
                    facing_right: true, shoot_timer: 0.0
                },
            ],
            projectiles: Vec::new(),
            music_enabled: true,
            music_step: 0,
            music_timer: 0.0,
            music_tempo: 15.0, // 15 frames per step = 4 steps per second at 60fps
        }
    }
    
    pub fn update(&mut self, input: u8) {
        // Handle title screen separately
        if self.game_state == GameState::TitleScreen {
            self.update_title_screen(input);
            return;
        }

        // Handle cutscene separately
        if self.game_state == GameState::Cutscene {
            self.update_cutscene(input);
            return;
        }

        // Simple player movement
        const PLAYER_SPEED: f32 = 4.0;
        const GRAVITY: f32 = 0.5;
        const JUMP_SPEED: f32 = -12.0;

        // Handle input
        let left = (input & 0x01) != 0;
        let right = (input & 0x02) != 0;
        let up = (input & 0x04) != 0; // Up arrow/W key
        let jump = (input & 0x10) != 0; // A button
        
        
        // Horizontal movement
        if left {
            self.player_vx = -PLAYER_SPEED;
            self.facing_right = false; // Face left when moving left
        } else if right {
            self.player_vx = PLAYER_SPEED;
            self.facing_right = true; // Face right when moving right
        } else {
            self.player_vx = 0.0;
        }
        
        // Variable jump height system
        let jump_input = jump || up;
        
        // Start new jump from ground
        if jump_input && self.on_ground {
            self.jumping = true;
            self.jump_button_held = true;
            self.jump_hold_time = 0.0;
            self.player_vy = JUMP_SPEED * 0.3; // Start with weak jump
            self.on_ground = false;
            // Don't play sound immediately - wait to see if it's short or long jump
        }
        
        // Continue adding power while holding jump and in air
        if jump_input && self.jump_button_held && self.jumping {
            self.jump_hold_time += 1.0;
            
            // Play long jump sound on frame 3 of holding
            if self.jump_hold_time == 3.0 {
                self.trigger_sound(0); // Long jump sound (ID 0)
            }
            
            // Add upward velocity for first 12 frames
            if self.jump_hold_time <= 12.0 {
                self.player_vy -= 0.6; // Add upward velocity each frame
            }
        }
        
        // Handle jump button release
        if !jump_input && self.jump_button_held {
            self.jump_button_held = false;
            
            // If released quickly (within 2 frames), play short jump sound
            if self.jumping && self.jump_hold_time <= 2.0 {
                self.trigger_sound(2); // Short jump sound (ID 2) 
            }
        }
        
        // Reset jumping state when landing
        if self.on_ground {
            self.jumping = false;
            self.jump_button_held = false;
            self.jump_hold_time = 0.0;
        }
        
        // Apply gravity
        self.player_vy += GRAVITY;
        
        // Update position with collision
        self.update_physics();
        
        // Update animation
        self.update_animation();
        
        // Update camera to follow player
        self.camera_x = (self.player_x - 160.0).max(0.0);
        let max_camera_x = self.level_width - 320.0;
        if self.camera_x > max_camera_x {
            self.camera_x = max_camera_x;
        }
        
        // Check for pitfall death
        if self.check_for_pitfall(self.player_x, self.player_y) {
            // Player fell into a pitfall - respawn at start
            self.player_x = 50.0;
            self.player_y = 150.0;
            self.player_vx = 0.0;
            self.player_vy = 0.0;
            self.camera_x = 0.0;
            self.camera_y = 0.0;
        }
        
        // Simple level reset with B button
        if input & 0x20 != 0 && self.on_ground { // B button to reset
            self.player_x = 50.0;
            self.player_y = 150.0;
            self.player_vx = 0.0;
            self.player_vy = 0.0;
            self.camera_x = 0.0;
            self.camera_y = 0.0;
        }

        // Update hexagnomes
        self.update_hexagnomes();

        // Update projectiles
        self.update_projectiles();

        // Update background music
        self.update_music();
    }
    
    fn update_physics(&mut self) {
        // Try horizontal movement first
        let new_x = self.player_x + self.player_vx;
        if !self.check_tile_collision(new_x, self.player_y) {
            self.player_x = new_x;
        }
        
        // Keep player in bounds horizontally
        if self.player_x < 8.0 {
            self.player_x = 8.0;
        }
        if self.player_x > self.level_width - 8.0 {
            self.player_x = self.level_width - 8.0;
        }
        
        // Try vertical movement
        let new_y = self.player_y + self.player_vy;
        
        // Check for collision at new vertical position
        if !self.check_tile_collision(self.player_x, new_y) {
            // No collision, move freely
            self.player_y = new_y;
            
            // Check if we're still on ground by looking slightly below
            let ground_check_y = self.player_y + 8.0;
            self.on_ground = self.check_tile_collision(self.player_x, ground_check_y);
        } else {
            // Collision detected
            if self.player_vy > 0.0 {
                // Moving down - landing
                if !self.on_ground {
                    self.trigger_sound(1); // Landing sound (ID 1) - only if we weren't already on ground
                }
                self.player_vy = 0.0;
                self.on_ground = true;
            } else if self.player_vy < 0.0 {
                // Moving up - hit ceiling
                self.player_vy = 0.0;
            }
        }
        
        // Simple fallback - prevent falling through bottom of world
        if self.player_y > 230.0 {
            self.player_y = 230.0;
            self.player_vy = 0.0;
            self.on_ground = true;
        }
    }
    
    fn check_tile_collision(&self, x: f32, y: f32) -> bool {
        // Check the four corners of the player sprite (16x16 centered on x,y)
        let corners = [
            (x - 7.0, y - 7.0), // Top-left
            (x + 7.0, y - 7.0), // Top-right  
            (x - 7.0, y + 7.0), // Bottom-left
            (x + 7.0, y + 7.0), // Bottom-right
        ];
        
        for (corner_x, corner_y) in corners.iter() {
            if *corner_x < 0.0 || *corner_y < 0.0 {
                continue;
            }
            
            let tile_x = (*corner_x / 16.0) as usize;
            let tile_y = (*corner_y / 16.0) as usize;
            
            if tile_x < 200 && tile_y < 15 {
                let tile_type = self.tiles[tile_y][tile_x];
                // Only solid blocks (1) and platforms (2) cause collision
                // Air (0), pitfalls (3), and passages (4) allow movement
                if tile_type == 1 || tile_type == 2 {
                    return true; // Collision detected
                }
            }
        }
        
        false // No collision
    }
    
    fn check_for_pitfall(&self, x: f32, y: f32) -> bool {
        // Check if player is standing on a pitfall
        let tile_x = (x / 16.0) as usize;
        let tile_y = (y / 16.0) as usize;
        
        if tile_x < 200 && tile_y < 15 {
            return self.tiles[tile_y][tile_x] == 3; // Pitfall tile
        }
        false
    }
    
    fn check_for_passage(&self, x: f32, y: f32) -> bool {
        // Check if player is in a passage
        let tile_x = (x / 16.0) as usize;
        let tile_y = (y / 16.0) as usize;
        
        if tile_x < 200 && tile_y < 15 {
            return self.tiles[tile_y][tile_x] == 4; // Passage tile
        }
        false
    }
    
    fn get_tile_collision_type(&self, tile_id: u8) -> TileCollisionType {
        match tile_id {
            0 => TileCollisionType::None,           // Air
            1 => TileCollisionType::Solid,          // Solid block
            2 => TileCollisionType::OneWayPlatform, // Platform
            3 => TileCollisionType::Pitfall,        // Pitfall (deadly)
            4 => TileCollisionType::Passage,        // Passage (leads down)
            5 => TileCollisionType::Water,          // Water
            6 => TileCollisionType::SwimThrough,    // Swim-through tunnel
            _ => TileCollisionType::None,           // Default to air
        }
    }
    
    fn update_animation(&mut self) {
        // Always increment timer to see if function is called
        self.animation_timer += 1.0;
        
        // Simple frame-count based animation
        let is_walking = self.player_vx.abs() > 0.1 && self.on_ground;
        
        // Removed noisy animation debug logging
        
        if is_walking {
            // Timer already incremented above
            
            // 4-frame cycle: 0, 1, 0, 2 (idle, walk1, idle, walk2)
            // Change every 4 frames (about 0.067 seconds at 60 FPS)
            let frame_duration = 4.0;
            let cycle_frame = ((self.animation_timer / frame_duration) as u32) % 4;
            
            self.animation_frame = match cycle_frame {
                0 => 0, // idle
                1 => 1, // walk1  
                2 => 0, // idle again
                3 => 2, // walk2
                _ => 0,
            };
            
            // Animation debug logging removed for performance
        } else {
            // Standing still or in air - use idle frame
            self.animation_frame = 0;
        }
    }
    
    pub fn get_animation_frame(&self) -> u32 {
        self.animation_frame
    }
    
    pub fn is_facing_right(&self) -> bool {
        self.facing_right
    }
    
    pub fn get_sprite_data(&self, frame: u32) -> &[[u8; 16]; 16] {
        match frame {
            0 => &HAMBERT_IDLE_SPRITE,
            1 => &HAMBERT_WALK1_SPRITE, 
            2 => &HAMBERT_WALK2_SPRITE,
            _ => &HAMBERT_IDLE_SPRITE,
        }
    }
    
    pub fn get_player_position(&self) -> (f32, f32) {
        (self.player_x, self.player_y)
    }
    
    pub fn get_camera_position(&self) -> (f32, f32) {
        (self.camera_x, self.camera_y)
    }
    
    pub fn get_current_level_name(&self) -> &str {
        "Simple Platformer"
    }
    
    pub fn get_tile_at(&self, tile_x: usize, tile_y: usize) -> u8 {
        if tile_x < 200 && tile_y < 15 {
            self.tiles[tile_y][tile_x]
        } else {
            0 // Air outside bounds
        }
    }

    pub fn get_tiles(&self) -> Vec<Vec<u8>> {
        self.tiles.iter().map(|row| row.to_vec()).collect()
    }

    pub fn get_tile_pixels(&self, tile_id: u8) -> Option<&[[u8; TILESET_TILE_SIZE]; TILESET_TILE_SIZE]> {
        if (tile_id as usize) < TILESET_TILE_COUNT {
            Some(&TILESET_DATA[tile_id as usize])
        } else {
            None
        }
    }

    pub fn get_tileset(&self) -> Vec<[[u8; 16]; 16]> {
        TILESET_DATA.to_vec()
    }

    // Hexagnome enemy/NPC management
    pub fn get_hexagnomes(&self) -> &[Hexagnome] {
        &self.hexagnomes
    }

    pub fn get_hexagnome_sprite() -> &'static [[u8; 26]; 32] {
        &HEXAGNOME_SPRITE
    }

    pub fn get_projectile_sprite() -> &'static [[u8; 6]; 6] {
        &HEXAGON_PROJECTILE_SPRITE
    }

    // Sound effect management
    pub fn get_pending_sounds(&self) -> Vec<u32> {
        self.pending_sounds.clone()
    }
    
    pub fn clear_pending_sounds(&mut self) {
        self.pending_sounds.clear();
    }
    
    fn trigger_sound(&mut self, sound_id: u32) {
        self.pending_sounds.push(sound_id);
    }

    fn update_hexagnomes(&mut self) {
        const HEXAGNOME_SPEED: f32 = 1.0;
        const SHOOT_INTERVAL: f32 = 90.0; // Shoot every 1.5 seconds at 60fps

        for hexagnome in &mut self.hexagnomes {
            // Move back and forth within patrol range
            hexagnome.x += hexagnome.vx * HEXAGNOME_SPEED;

            // Check patrol bounds and reverse direction
            if hexagnome.x <= hexagnome.patrol_left {
                hexagnome.x = hexagnome.patrol_left;
                hexagnome.vx = 1.0;
            } else if hexagnome.x >= hexagnome.patrol_right {
                hexagnome.x = hexagnome.patrol_right;
                hexagnome.vx = -1.0;
            }

            // Face the direction where the player is
            if self.player_x < hexagnome.x {
                hexagnome.facing_right = false; // Player is to the left
            } else {
                hexagnome.facing_right = true; // Player is to the right
            }

            // Update shoot timer and spawn projectiles
            hexagnome.shoot_timer += 1.0;
            if hexagnome.shoot_timer >= SHOOT_INTERVAL {
                hexagnome.shoot_timer = 0.0;

                // Spawn projectile toward player
                let projectile_vx = if hexagnome.facing_right { 3.0 } else { -3.0 };
                self.projectiles.push(Projectile {
                    x: hexagnome.x,
                    y: hexagnome.y,
                    vx: projectile_vx,
                    active: true,
                });
            }
        }
    }

    fn update_projectiles(&mut self) {
        // Update all projectiles
        for projectile in &mut self.projectiles {
            if projectile.active {
                projectile.x += projectile.vx;

                // Deactivate if off screen (far left or far right)
                if projectile.x < -50.0 || projectile.x > self.level_width + 50.0 {
                    projectile.active = false;
                }
            }
        }

        // Remove inactive projectiles
        self.projectiles.retain(|p| p.active);
    }

    pub fn get_projectiles(&self) -> &[Projectile] {
        &self.projectiles
    }

    fn update_music(&mut self) {
        if !self.music_enabled {
            return;
        }

        self.music_timer += 1.0;

        if self.music_timer >= self.music_tempo {
            self.music_timer = 0.0;
            self.music_step = (self.music_step + 1) % 32; // 32-step loop
        }
    }

    pub fn get_music_step(&self) -> usize {
        self.music_step
    }

    pub fn is_music_enabled(&self) -> bool {
        self.music_enabled
    }

    fn update_title_screen(&mut self, input: u8) {
        // Update blink timer for "PRESS START" animation
        self.title_blink_timer += 1.0;

        // Detect button press edge
        let a_button_pressed = (input & 0x10) != 0 && (self.previous_input & 0x10) == 0;
        let b_button_pressed = (input & 0x20) != 0 && (self.previous_input & 0x20) == 0;

        if a_button_pressed || b_button_pressed {
            // Start intro cutscene before gameplay
            self.start_cutscene(CutsceneType::Intro);
        }

        // Store input for next frame
        self.previous_input = input;
    }

    fn update_cutscene(&mut self, input: u8) {
        let mut sound_to_play: Option<u32> = None;
        let mut advance_screen = false;
        let mut cutscene_finished = false;

        if let Some(ref mut cutscene) = self.current_cutscene {
            // Update text scrolling
            cutscene.text_scroll_offset += cutscene.text_scroll_speed;

            // Update typing effect - reveal characters one at a time
            let old_char_index = cutscene.text_char_index;
            cutscene.text_char_timer += cutscene.chars_per_frame;
            if cutscene.text_char_timer >= 1.0 {
                cutscene.text_char_timer -= 1.0;
                cutscene.text_char_index += 1;

                // Play typing sound when a new character appears
                if cutscene.text_char_index > old_char_index {
                    if let Some(current_screen) = cutscene.screens.get(cutscene.current_screen) {
                        // Count total characters to see if we're still in text range
                        let total_chars: usize = current_screen.text_lines.iter().map(|s| s.len()).sum();
                        if cutscene.text_char_index <= total_chars {
                            // Queue text blip sound to play after borrow
                            sound_to_play = Some(7);
                        }
                    }
                }
            }

            // Detect button press edge
            let a_button_pressed = (input & 0x10) != 0 && (self.previous_input & 0x10) == 0;
            let b_button_pressed = (input & 0x20) != 0 && (self.previous_input & 0x20) == 0;

            if a_button_pressed || b_button_pressed {
                // Check if advancing to a valid screen or ending cutscene
                if cutscene.current_screen + 1 < cutscene.screens.len() {
                    advance_screen = true;
                    // Queue screen transition sound
                    if let Some(next_sound_id) = cutscene.screens[cutscene.current_screen + 1].sound_id {
                        sound_to_play = Some(next_sound_id);
                    }
                } else {
                    // On the last screen - end the cutscene
                    cutscene_finished = true;
                }
            }
        }

        // Handle screen advancement after borrow is released
        if advance_screen {
            if let Some(ref mut cutscene) = self.current_cutscene {
                cutscene.current_screen += 1;
                cutscene.text_scroll_offset = 0.0;
                cutscene.text_char_index = 0;
                cutscene.text_char_timer = 0.0;
            }
        }

        // Handle cutscene end
        if cutscene_finished {
            self.current_cutscene = None;
            self.game_state = GameState::Playing;
            // Reset player position for new game start
            self.player_x = 50.0;
            self.player_y = 180.0;
        }

        // Play any queued sound
        if let Some(sound_id) = sound_to_play {
            self.trigger_sound(sound_id);
        }

        // Store input for next frame
        self.previous_input = input;
    }

    fn start_cutscene(&mut self, cutscene_type: CutsceneType) {
        let cutscene = match cutscene_type {
            CutsceneType::Intro => Self::create_intro_cutscene(),
            CutsceneType::Interlude1 => Self::create_interlude1_cutscene(),
            CutsceneType::Interlude2 => Self::create_interlude2_cutscene(),
            CutsceneType::Interlude3 => Self::create_interlude3_cutscene(),
            CutsceneType::Interlude4 => Self::create_interlude4_cutscene(),
            CutsceneType::Ending => Self::create_ending_cutscene(),
        };

        self.current_cutscene = Some(cutscene);
        self.game_state = GameState::Cutscene;
    }

    pub fn get_game_state(&self) -> &GameState {
        &self.game_state
    }

    pub fn should_show_press_start(&self) -> bool {
        // Blink "PRESS START" every 30 frames (0.5 seconds at 60fps)
        (self.title_blink_timer as u32 / 30) % 2 == 0
    }

    pub fn get_title_logo(&self) -> &[[u8; 80]; 32] {
        &TITLE_LOGO
    }

    pub fn get_current_cutscene(&self) -> Option<&Cutscene> {
        self.current_cutscene.as_ref()
    }

    // Cutscene creation functions
    fn create_intro_cutscene() -> Cutscene {
        let screens = vec![
            CutsceneScreen {
                image: CUTSCENE_HAMBERT_FACE,
                text_lines: vec![
                    "LONG AGO, IN A PEACEFUL FOREST...".to_string(),
                    "LIVED A BRAVE HAMSTER NAMED HAMBERT.".to_string(),
                ],
                sound_id: Some(6), // Laugh sound
            },
            CutsceneScreen {
                image: CUTSCENE_HEXAGNOME,
                text_lines: vec![
                    "BUT ONE DAY, THE EVIL HEXAGNOMES".to_string(),
                    "INVADED THE FOREST!".to_string(),
                ],
                sound_id: Some(4), // Enemy sound
            },
            CutsceneScreen {
                image: CUTSCENE_HAMBERT_HERO,
                text_lines: vec![
                    "NOW HAMBERT MUST FIGHT BACK".to_string(),
                    "AND SAVE HIS HOME!".to_string(),
                    "".to_string(),
                    "PRESS START TO BEGIN".to_string(),
                ],
                sound_id: Some(0), // Jump sound
            },
        ];

        Cutscene {
            cutscene_type: CutsceneType::Intro,
            screens,
            current_screen: 0,
            text_scroll_offset: 0.0,
            text_scroll_speed: 0.5,
            text_char_index: 0,
            text_char_timer: 0.0,
            chars_per_frame: 1.5,  // 1.5 characters per frame = faster typing
        }
    }

    fn create_interlude1_cutscene() -> Cutscene {
        let screens = vec![
            CutsceneScreen {
                image: CUTSCENE_HAMBERT_FACE,
                text_lines: vec![
                    "LEVEL 1 COMPLETE!".to_string(),
                    "BUT THE JOURNEY CONTINUES...".to_string(),
                ],
                sound_id: Some(3),
            },
        ];

        Cutscene {
            cutscene_type: CutsceneType::Interlude1,
            screens,
            current_screen: 0,
            text_scroll_offset: 0.0,
            text_scroll_speed: 0.5,
            text_char_index: 0,
            text_char_timer: 0.0,
            chars_per_frame: 1.5,
        }
    }

    fn create_interlude2_cutscene() -> Cutscene {
        let screens = vec![
            CutsceneScreen {
                image: CUTSCENE_HAMBERT_FACE,
                text_lines: vec![
                    "LEVEL 2 COMPLETE!".to_string(),
                    "DEEPER INTO THE FOREST...".to_string(),
                ],
                sound_id: Some(3),
            },
        ];

        Cutscene {
            cutscene_type: CutsceneType::Interlude2,
            screens,
            current_screen: 0,
            text_scroll_offset: 0.0,
            text_scroll_speed: 0.5,
            text_char_index: 0,
            text_char_timer: 0.0,
            chars_per_frame: 1.5,
        }
    }

    fn create_interlude3_cutscene() -> Cutscene {
        let screens = vec![
            CutsceneScreen {
                image: CUTSCENE_HAMBERT_FACE,
                text_lines: vec![
                    "LEVEL 3 COMPLETE!".to_string(),
                    "THE END IS NEAR...".to_string(),
                ],
                sound_id: Some(3),
            },
        ];

        Cutscene {
            cutscene_type: CutsceneType::Interlude3,
            screens,
            current_screen: 0,
            text_scroll_offset: 0.0,
            text_scroll_speed: 0.5,
            text_char_index: 0,
            text_char_timer: 0.0,
            chars_per_frame: 1.5,
        }
    }

    fn create_interlude4_cutscene() -> Cutscene {
        let screens = vec![
            CutsceneScreen {
                image: CUTSCENE_HAMBERT_FACE,
                text_lines: vec![
                    "LEVEL 4 COMPLETE!".to_string(),
                    "PREPARE FOR THE FINAL BATTLE!".to_string(),
                ],
                sound_id: Some(3),
            },
        ];

        Cutscene {
            cutscene_type: CutsceneType::Interlude4,
            screens,
            current_screen: 0,
            text_scroll_offset: 0.0,
            text_scroll_speed: 0.5,
            text_char_index: 0,
            text_char_timer: 0.0,
            chars_per_frame: 1.5,
        }
    }

    fn create_ending_cutscene() -> Cutscene {
        let screens = vec![
            CutsceneScreen {
                image: CUTSCENE_HAMBERT_HERO,
                text_lines: vec![
                    "THE HEXAGNOMES HAVE BEEN DEFEATED!".to_string(),
                ],
                sound_id: Some(6),
            },
            CutsceneScreen {
                image: CUTSCENE_HAMBERT_FACE,
                text_lines: vec![
                    "PEACE RETURNS TO THE FOREST.".to_string(),
                    "HAMBERT IS THE HERO!".to_string(),
                ],
                sound_id: Some(3),
            },
            CutsceneScreen {
                image: CUTSCENE_HAMBERT_HERO,
                text_lines: vec![
                    "THANK YOU FOR PLAYING!".to_string(),
                    "".to_string(),
                    "THE END".to_string(),
                ],
                sound_id: None,
            },
        ];

        Cutscene {
            cutscene_type: CutsceneType::Ending,
            screens,
            current_screen: 0,
            text_scroll_offset: 0.0,
            text_scroll_speed: 0.5,
            text_char_index: 0,
            text_char_timer: 0.0,
            chars_per_frame: 1.5,
        }
    }
}

// Title screen logo - 80x32 pixels
// Simple "HAMBERT'S ADVENTURE" style logo
const TITLE_LOGO: [[u8; 80]; 32] = [
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    [0,0,0,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,0,0,0,0],
    [0,0,0,10,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,10,0,0,0,0],
    [0,0,0,10,8,22,22,22,22,22,22,22,8,8,8,8,8,8,8,8,22,22,22,22,22,22,22,22,8,8,8,8,22,22,22,22,22,22,8,8,8,22,22,22,22,22,22,22,8,8,22,22,22,22,22,22,8,8,22,22,22,22,22,22,22,8,8,22,22,22,22,22,22,22,8,10,0,0,0,0],
    [0,0,0,10,8,22,22,22,22,22,22,22,8,8,8,8,8,8,8,8,22,22,22,22,22,22,22,22,8,8,8,8,22,22,22,22,22,22,8,8,8,22,22,22,22,22,22,22,8,8,22,22,22,22,22,22,8,8,22,22,22,22,22,22,22,8,8,22,22,22,22,22,22,22,8,10,0,0,0,0],
    [0,0,0,10,8,22,22,8,8,8,8,22,22,8,8,8,8,8,8,8,22,22,8,8,8,8,22,22,8,8,8,8,22,22,8,8,22,22,8,8,8,22,22,8,8,8,8,8,8,8,22,22,8,8,22,22,8,8,22,22,8,8,8,8,8,8,8,22,22,8,8,8,8,8,8,10,0,0,0,0],
    [0,0,0,10,8,22,22,8,8,8,8,22,22,8,8,8,8,8,8,8,22,22,8,8,8,8,22,22,8,8,8,8,22,22,8,8,8,8,8,8,8,22,22,8,8,8,8,8,8,8,22,22,8,8,22,22,8,8,22,22,8,8,8,8,8,8,8,22,22,8,8,8,8,8,8,10,0,0,0,0],
    [0,0,0,10,8,22,22,8,8,8,8,22,22,8,8,8,8,8,8,8,22,22,8,8,8,8,22,22,8,8,8,8,22,22,8,8,8,8,8,8,8,22,22,8,8,8,8,8,8,8,22,22,8,8,22,22,8,8,22,22,8,8,8,8,8,8,8,22,22,8,8,8,8,8,8,10,0,0,0,0],
    [0,0,0,10,8,22,22,22,22,22,22,22,8,8,8,8,8,8,8,8,22,22,8,8,8,8,22,22,8,8,8,8,22,22,8,8,8,8,8,8,8,22,22,22,22,22,22,8,8,8,22,22,8,8,22,22,8,8,22,22,22,22,22,8,8,8,8,22,22,22,22,22,22,8,8,10,0,0,0,0],
    [0,0,0,10,8,22,22,22,22,22,22,22,8,8,8,8,8,8,8,8,22,22,8,8,8,8,22,22,8,8,8,8,22,22,8,8,8,8,8,8,8,22,22,22,22,22,22,8,8,8,22,22,8,8,22,22,8,8,22,22,22,22,22,8,8,8,8,22,22,22,22,22,22,8,8,10,0,0,0,0],
    [0,0,0,10,8,22,22,8,8,8,8,22,22,8,8,8,8,8,8,8,22,22,8,8,8,8,22,22,8,8,8,8,22,22,8,8,22,22,8,8,8,22,22,8,8,8,8,8,8,8,22,22,8,8,22,22,8,8,22,22,8,8,8,8,8,8,8,22,22,8,8,8,8,8,8,10,0,0,0,0],
    [0,0,0,10,8,22,22,8,8,8,8,22,22,8,8,8,8,8,8,8,22,22,8,8,8,8,22,22,8,8,8,8,22,22,8,8,22,22,8,8,8,22,22,8,8,8,8,8,8,8,22,22,8,8,22,22,8,8,22,22,8,8,8,8,8,8,8,22,22,8,8,8,8,8,8,10,0,0,0,0],
    [0,0,0,10,8,22,22,8,8,8,8,22,22,8,8,8,8,8,8,8,22,22,8,8,8,8,22,22,8,8,8,8,22,22,8,8,22,22,8,8,8,22,22,8,8,8,8,8,8,8,22,22,22,22,22,22,8,8,22,22,8,8,8,8,8,8,8,22,22,8,8,8,8,8,8,10,0,0,0,0],
    [0,0,0,10,8,22,22,8,8,8,8,22,22,8,8,8,8,8,8,8,22,22,22,22,22,22,22,22,8,8,8,8,22,22,8,8,22,22,8,8,8,22,22,22,22,22,22,22,8,8,22,22,22,22,22,22,8,8,22,22,22,22,22,22,22,8,8,22,22,22,22,22,22,22,8,10,0,0,0,0],
    [0,0,0,10,8,22,22,8,8,8,8,22,22,8,8,8,8,8,8,8,22,22,22,22,22,22,22,22,8,8,8,8,22,22,8,8,22,22,8,8,8,22,22,22,22,22,22,22,8,8,22,22,22,22,22,22,8,8,22,22,22,22,22,22,22,8,8,22,22,22,22,22,22,22,8,10,0,0,0,0],
    [0,0,0,10,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,10,0,0,0,0],
    [0,0,0,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
];

// Cutscene images - 64x64 pixels
// Simple Hambert face portrait
const CUTSCENE_HAMBERT_FACE: [[u8; 64]; 64] = {
    let mut img = [[0u8; 64]; 64];
    // Simple brown circle for face
    let center = 32;
    let radius = 20;
    let mut y = 0;
    while y < 64 {
        let mut x = 0;
        while x < 64 {
            let dx = (x as i32 - center as i32).abs();
            let dy = (y as i32 - center as i32).abs();
            if dx * dx + dy * dy < radius * radius {
                img[y][x] = 22; // Brown color
            }
            x += 1;
        }
        y += 1;
    }
    img
};

// Hexagnome enemy portrait
const CUTSCENE_HEXAGNOME: [[u8; 64]; 64] = {
    let mut img = [[0u8; 64]; 64];
    // Simple gray hexagon shape
    let center = 32;
    let size = 18;
    let mut y = 0;
    while y < 64 {
        let mut x = 0;
        while x < 64 {
            let dx = (x as i32 - center as i32).abs();
            let dy = (y as i32 - center as i32).abs();
            if dx < size && dy < size {
                img[y][x] = 8; // Gray color
            }
            x += 1;
        }
        y += 1;
    }
    img
};

// Hambert hero pose
const CUTSCENE_HAMBERT_HERO: [[u8; 64]; 64] = {
    let mut img = [[0u8; 64]; 64];
    // Simple brown circle with "heroic" star
    let center = 32;
    let radius = 20;
    let mut y = 0;
    while y < 64 {
        let mut x = 0;
        while x < 64 {
            let dx = (x as i32 - center as i32).abs();
            let dy = (y as i32 - center as i32).abs();
            if dx * dx + dy * dy < radius * radius {
                img[y][x] = 22; // Brown color
            }
            // Add star in center
            if (dx < 3 && dy < 8) || (dy < 3 && dx < 8) {
                img[y][x] = 37; // Yellow star
            }
            x += 1;
        }
        y += 1;
    }
    img
};