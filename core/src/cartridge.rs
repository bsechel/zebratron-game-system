use wasm_bindgen::prelude::*;

// Sound effect IDs for the Hambert game
#[derive(Clone, Copy)]
pub enum SoundEffect {
    Jump = 0,
    Land = 1,
    Collect = 2,
    EnemyHit = 3,
    ShurikenThrow = 4,
}

// Audio commands that cartridges can send to the console
pub trait AudioCommands {
    fn play_sound_effect(&mut self, sound_id: u32);
    fn play_music(&mut self, music_id: u32);
    fn stop_music(&mut self);
    fn set_music_volume(&mut self, volume: f32);
}

#[derive(Clone, Copy, PartialEq)]
pub enum EntityType {
    Player,
    Enemy,
    Ninja,
    Platform,
    Projectile,
    Shuriken,
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
            EntityType::Ninja => (20.0, 32.0),   // Taller, more figure-like
            EntityType::Platform => (64.0, 16.0),
            EntityType::Projectile => (8.0, 8.0),
            EntityType::Shuriken => (12.0, 12.0), // Spinning projectile
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
                EntityType::Ninja => 2,
                _ => 1,
            },
            animation_frame: 0,
            animation_timer: 0.0,
        }
    }
}

// Cartridge trait - all games must implement this
pub trait Cartridge {
    fn init(&mut self) -> Result<(), String>;
    fn update(&mut self, input: &GameInput) -> Result<(), String>;
    fn get_sprites(&self) -> &[SpriteData];
    fn get_camera_pos(&self) -> (f32, f32);
    fn reset(&mut self);
}

// Input state passed from the system to cartridge
#[derive(Default)]
pub struct GameInput {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub a: bool,
    pub b: bool,
    pub start: bool,
    pub select: bool,
}

// Sprite data for rendering - cartridge communicates with PPU through this
#[derive(Clone)]
pub struct SpriteData {
    pub x: f32,
    pub y: f32,
    pub sprite_id: u32,
    pub active: bool,
}

#[derive(Clone, Copy, PartialEq)]
pub enum GameState {
    Intro,
    Playing,
    Interlude,
}

// The Hambert cartridge - extracted game logic
#[wasm_bindgen]
pub struct HambertCartridge {
    entities: Vec<Entity>,
    player_id: usize,
    camera_x: f32,
    camera_y: f32,
    world_width: f32,
    world_height: f32,
    pending_shuriken: Vec<Entity>,
    frame_count: u64,
    pending_sounds: Vec<SoundEffect>,

    // Game state management
    game_state: GameState,
    text_timer: f32,
    text_index: usize,
    current_level: u32,
}

#[wasm_bindgen]
impl HambertCartridge {
    #[wasm_bindgen(constructor)]
    pub fn new() -> HambertCartridge {
        let mut cartridge = HambertCartridge {
            entities: Vec::new(),
            player_id: 0,
            camera_x: 0.0,
            camera_y: 0.0,
            world_width: 2000.0,
            world_height: 480.0,
            pending_shuriken: Vec::new(),
            frame_count: 0,
            pending_sounds: Vec::new(),

            // Start with intro screen
            game_state: GameState::Intro,
            text_timer: 0.0,
            text_index: 0,
            current_level: 1,
        };

        // Initialize the game world
        cartridge.init_world();
        cartridge
    }

    fn init_world(&mut self) {
        // Create player entity (Hambert) - sprite ID 0
        // Start Hambert at ground level (200 - sprite height)
        let player = Entity::new(EntityType::Player, 100.0, 180.0, 0);
        self.entities.push(player);
        self.player_id = 0;

        // Add some platforms for Hambert to jump on - sprite ID 1
        // Ground level is at Y=200, so platforms should be above that (lower Y values)
        self.entities.push(Entity::new(EntityType::Platform, 200.0, 170.0, 1));
        self.entities.push(Entity::new(EntityType::Platform, 400.0, 140.0, 1));
        self.entities.push(Entity::new(EntityType::Platform, 600.0, 110.0, 1));
        self.entities.push(Entity::new(EntityType::Platform, 800.0, 130.0, 1));
        self.entities.push(Entity::new(EntityType::Platform, 1000.0, 100.0, 1));
        self.entities.push(Entity::new(EntityType::Platform, 1200.0, 90.0, 1));
        self.entities.push(Entity::new(EntityType::Platform, 1400.0, 120.0, 1));
        self.entities.push(Entity::new(EntityType::Platform, 1600.0, 80.0, 1));
        self.entities.push(Entity::new(EntityType::Platform, 1800.0, 110.0, 1));

        // Add some ninja enemies - sprite ID 3
        // Place ninjas on platforms and ground level
        self.entities.push(Entity::new(EntityType::Ninja, 300.0, 150.0, 3)); // On first platform
        self.entities.push(Entity::new(EntityType::Ninja, 500.0, 168.0, 3)); // On ground
        self.entities.push(Entity::new(EntityType::Ninja, 900.0, 98.0, 3));  // On high platform
        self.entities.push(Entity::new(EntityType::Ninja, 1300.0, 88.0, 3)); // On higher platform
    }

    pub fn update_game(&mut self, up: bool, down: bool, left: bool, right: bool) {
        self.frame_count += 1;

        match self.game_state {
            GameState::Intro => {
                self.update_intro_screen(up, down, left, right);
            },
            GameState::Playing => {
                // Handle input
                self.handle_input(up, down, left, right);

                // Update physics
                self.update_physics();

                // Spawn ninjas
                self.update_ninja_spawning();

                // Add pending shuriken
                self.entities.extend(self.pending_shuriken.drain(..));

                // Update camera to follow player
                self.update_camera();
            },
            GameState::Interlude => {
                self.update_interlude_screen(up, down, left, right);
            },
        }
    }

    fn handle_input(&mut self, up: bool, _down: bool, left: bool, right: bool) {
        if self.player_id >= self.entities.len() {
            return;
        }

        let player = &mut self.entities[self.player_id];

        // Horizontal movement (slower acceleration for deliberate control)
        if left {
            player.vel_x -= 0.2;
        }
        if right {
            player.vel_x += 0.2;
        }

        // Jumping (higher jump, more satisfying)
        if up && player.on_ground {
            player.vel_y = -6.5;
            player.on_ground = false;
            self.pending_sounds.push(SoundEffect::Jump);
        }

        // Clamp horizontal velocity (slower max speed)
        player.vel_x = player.vel_x.max(-2.8).min(2.8);
    }

    fn update_physics(&mut self) {
        const GRAVITY: f32 = 0.15;  // Much slower gravity for deliberate jumps
        const MAX_FALL_SPEED: f32 = 3.5;  // Even slower terminal velocity

        // Get player position for ninja AI
        let _player_pos = if self.player_id < self.entities.len() {
            Some((self.entities[self.player_id].x, self.entities[self.player_id].y))
        } else {
            None
        };

        for entity in &mut self.entities {
            if !entity.active {
                continue;
            }

            match entity.entity_type {
                EntityType::Player | EntityType::Ninja => {
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

                    // Apply friction when on ground (increased for more control)
                    if entity.on_ground {
                        entity.vel_x *= 0.75;  // More friction on ground
                    } else {
                        entity.vel_x *= 0.96;  // Slight air resistance
                    }

                    // Keep within world bounds
                    entity.x = entity.x.max(0.0).min(self.world_width - entity.width);
                }
                EntityType::Shuriken => {
                    // Shuriken physics
                    entity.x += entity.vel_x;
                    entity.y += entity.vel_y;
                    entity.vel_y += 0.1; // Reduced gravity from 0.2 to 0.1

                    // Rotate animation
                    entity.animation_timer += 1.0;
                    if entity.animation_timer >= 8.0 {
                        entity.animation_frame = (entity.animation_frame + 1) % 4;
                        entity.animation_timer = 0.0;
                    }

                    // Remove if off screen
                    if entity.x < -50.0 || entity.x > self.world_width + 50.0 || entity.y > self.world_height + 50.0 {
                        entity.active = false;
                    }
                }
                _ => {}
            }
        }

        // Platform collision detection
        self.check_platform_collisions();

        // Ground collision - simple flat ground at Y=200
        for entity in &mut self.entities {
            if matches!(entity.entity_type, EntityType::Player | EntityType::Ninja) {
                let ground_level = 200.0;
                if entity.y + entity.height >= ground_level {
                    entity.y = ground_level - entity.height;
                    entity.vel_y = 0.0;
                    entity.on_ground = true;
                }
            }
        }
    }

    fn check_platform_collisions(&mut self) {
        let mut platforms = Vec::new();
        for (i, entity) in self.entities.iter().enumerate() {
            if entity.entity_type == EntityType::Platform && entity.active {
                platforms.push((i, entity.clone()));
            }
        }

        for entity in &mut self.entities {
            if !matches!(entity.entity_type, EntityType::Player | EntityType::Ninja) {
                continue;
            }

            entity.on_ground = false;

            for (_, platform) in &platforms {
                if entity.x < platform.x + platform.width &&
                   entity.x + entity.width > platform.x &&
                   entity.y + entity.height > platform.y &&
                   entity.y + entity.height < platform.y + platform.height + 10.0 &&
                   entity.vel_y >= 0.0 {
                    entity.y = platform.y - entity.height;
                    entity.vel_y = 0.0;
                    entity.on_ground = true;
                    break;
                }
            }
        }
    }

    fn update_ninja_spawning(&mut self) {
        if self.frame_count % 300 == 0 {
            let spawn_x = if self.entities.len() % 2 == 0 { 0.0 } else { self.world_width - 20.0 };
            let ninja = Entity::new(EntityType::Ninja, spawn_x, 100.0, 3); // sprite ID 3 for ninja
            self.entities.push(ninja);
        }

        // Ninja AI and shuriken throwing
        if let Some((player_x, player_y)) = self.get_player_position() {
            for entity in &mut self.entities {
                if entity.entity_type == EntityType::Ninja && entity.active {
                    let dx = player_x - entity.x;
                    let distance = dx.abs();

                    if distance > 50.0 {
                        if dx > 0.0 {
                            entity.vel_x += 0.05; // Reduced from 0.1 to 0.05 (half speed)
                        } else {
                            entity.vel_x -= 0.05; // Reduced from 0.1 to 0.05 (half speed)
                        }
                        entity.vel_x = entity.vel_x.max(-1.0).min(1.0); // Reduced max speed from 2.0 to 1.0
                    }

                    // Throw shuriken occasionally
                    if distance < 200.0 && distance > 50.0 && self.frame_count % 120 == 0 {
                        let mut shuriken = Entity::new(EntityType::Shuriken, entity.x + 10.0, entity.y + 10.0, 4); // sprite ID 4 for shuriken
                        let dy = player_y - entity.y;
                        let speed = 2.0; // Reduced from 4.0 to 2.0 (half speed)
                        let distance = (dx * dx + dy * dy).sqrt();

                        if distance > 0.0 {
                            shuriken.vel_x = (dx / distance) * speed;
                            shuriken.vel_y = (dy / distance) * speed;
                        }

                        self.pending_shuriken.push(shuriken);
                        self.pending_sounds.push(SoundEffect::ShurikenThrow);
                    }
                }
            }
        }
    }

    fn update_camera(&mut self) {
        if self.player_id < self.entities.len() {
            let player = &self.entities[self.player_id];

            // Follow player horizontally (immediate)
            let screen_center_x = 160.0; // SCREEN_WIDTH / 2
            self.camera_x = (player.x + player.width / 2.0 - screen_center_x)
                .max(0.0)
                .min(self.world_width - 320.0);

            // COMPLETELY FIXED VERTICAL CAMERA - no vertical following at all
            // Position camera to show the full game area
            // Hambert at Y=180, want him at screen Y=200 (near bottom), so camera_y = 180-200 = -20
            self.camera_y = -20.0; // Show platforms above and ground below
        }
    }

    fn get_player_position(&self) -> Option<(f32, f32)> {
        if self.player_id < self.entities.len() {
            let player = &self.entities[self.player_id];
            Some((player.x, player.y))
        } else {
            None
        }
    }

    // Public getters for PPU
    pub fn get_camera_x(&self) -> f32 {
        self.camera_x
    }

    pub fn get_camera_y(&self) -> f32 {
        self.camera_y
    }

    pub fn get_entity_count(&self) -> usize {
        self.entities.len()
    }

    pub fn get_entity_data(&self, index: usize) -> Option<js_sys::Object> {
        if index >= self.entities.len() {
            return None;
        }

        let entity = &self.entities[index];
        let obj = js_sys::Object::new();

        js_sys::Reflect::set(&obj, &"x".into(), &entity.x.into()).unwrap();
        js_sys::Reflect::set(&obj, &"y".into(), &entity.y.into()).unwrap();
        js_sys::Reflect::set(&obj, &"sprite_id".into(), &entity.sprite_id.into()).unwrap();
        js_sys::Reflect::set(&obj, &"active".into(), &entity.active.into()).unwrap();
        js_sys::Reflect::set(&obj, &"entity_type".into(), &(entity.entity_type as u32).into()).unwrap();

        Some(obj)
    }

    // Intro/Interlude screen methods
    fn update_intro_screen(&mut self, up: bool, down: bool, left: bool, right: bool) {
        // Update typewriter animation
        self.text_timer += 1.0 / 60.0; // Assuming 60fps

        let chars_per_second = 20.0; // Speed of typewriter effect
        let target_index = (self.text_timer * chars_per_second) as usize;
        let intro_text = "Get ready for a funny adventure!";

        if target_index > self.text_index && self.text_index < intro_text.len() {
            self.text_index = target_index.min(intro_text.len());
        }

        // Any key to continue to game after text is complete
        if (up || down || left || right) && self.text_index >= intro_text.len() {
            self.game_state = GameState::Playing;
            self.text_timer = 0.0;
            self.text_index = 0;
        }
    }

    fn update_interlude_screen(&mut self, up: bool, down: bool, left: bool, right: bool) {
        // Update typewriter animation
        self.text_timer += 1.0 / 60.0;

        let chars_per_second = 20.0;
        let target_index = (self.text_timer * chars_per_second) as usize;
        let interlude_text = &format!("Level {} Complete! Ready for more?", self.current_level);

        if target_index > self.text_index && self.text_index < interlude_text.len() {
            self.text_index = target_index.min(interlude_text.len());
        }

        // Any key to continue to next level after text is complete
        if (up || down || left || right) && self.text_index >= interlude_text.len() {
            self.current_level += 1;
            self.game_state = GameState::Playing;
            self.text_timer = 0.0;
            self.text_index = 0;
            // Could reset world or load new level here
        }
    }

    pub fn get_game_state(&self) -> u32 {
        match self.game_state {
            GameState::Intro => 0,
            GameState::Playing => 1,
            GameState::Interlude => 2,
        }
    }

    pub fn get_intro_text(&self) -> String {
        match self.game_state {
            GameState::Intro => {
                let full_text = "Get ready for a funny adventure!";
                full_text.chars().take(self.text_index).collect()
            },
            GameState::Interlude => {
                let full_text = format!("Level {} Complete! Ready for more?", self.current_level);
                full_text.chars().take(self.text_index).collect()
            },
            _ => String::new(),
        }
    }

    // Audio interface methods
    pub fn get_pending_sounds(&self) -> Vec<u32> {
        self.pending_sounds.iter().map(|&sound| sound as u32).collect()
    }

    pub fn clear_pending_sounds(&mut self) {
        self.pending_sounds.clear();
    }
}