use wasm_bindgen::prelude::*;
use std::collections::HashMap;

// Sound effect IDs for the Hambert game
#[derive(Clone, Copy)]
pub enum SoundEffect {
    Jump = 0,
    Land = 1,
    Collect = 2,
    EnemyHit = 3,
    ShurikenThrow = 4,
    Death = 5,
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
    Hexagnome,
    BloodGoblin,
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
    pub is_dying: bool,
    pub death_timer: f32,
    pub death_flash_timer: f32,
    pub facing_left: bool,
}

impl Entity {
    pub fn new(entity_type: EntityType, x: f32, y: f32, sprite_id: u32) -> Self {
        let (width, height) = match entity_type {
            EntityType::Player => (32.0, 28.0),  // Hambert size (larger)
            EntityType::Enemy => (24.0, 24.0),
            EntityType::Hexagnome => (20.0, 28.0),   // Hexagnome dimensions (scaled for performance)
            EntityType::BloodGoblin => (20.0, 38.0), // Blood goblin dimensions (20x38)
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
                EntityType::Hexagnome => 2,
                EntityType::BloodGoblin => 1,
                _ => 1,
            },
            animation_frame: 0,
            animation_timer: 0.0,
            is_dying: false,
            death_timer: 0.0,
            death_flash_timer: 0.0,
            facing_left: false,
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
    lives: u32,
    score: u32,
    invulnerability_timer: f32,
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
            lives: 3,
            score: 0,
            invulnerability_timer: 0.0,
        };

        // Initialize the game world
        cartridge.init_world();
        cartridge
    }

    fn init_world(&mut self) {
        // Create player entity (Hambert) - sprite ID 0
        // Start Hambert at ground level (200 - sprite height = 200 - 32 = 168)
        let player = Entity::new(EntityType::Player, 100.0, 168.0, 0);
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

        // Add collectible hamberries scattered throughout the level - sprite ID 6
        self.spawn_hamberries();

        // Add some hexagnome enemies - sprite ID 3
        // Place hexagnomes on platforms and ground level
        self.entities.push(Entity::new(EntityType::Hexagnome, 300.0, 150.0, 3)); // On first platform
        self.entities.push(Entity::new(EntityType::Hexagnome, 500.0, 168.0, 3)); // On ground
        self.entities.push(Entity::new(EntityType::Hexagnome, 900.0, 98.0, 3));  // On high platform
        self.entities.push(Entity::new(EntityType::Hexagnome, 1300.0, 88.0, 3)); // On higher platform
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

                // Spawn hexagnomes
                self.update_hexagnome_spawning();

                // Check for blood goblin spawning at level end
                self.check_blood_goblin_spawn();

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

        // Horizontal movement (faster acceleration for responsive walking, slower in air)
        let acceleration = if player.on_ground { 0.35 } else { 0.175 };  // Half speed in air
        if left {
            player.vel_x -= acceleration;
        }
        if right {
            player.vel_x += acceleration;
        }

        // Jumping (lower, more controlled jump)
        if up && player.on_ground {
            player.vel_y = -4.0;  // Reduced from -5.0 for lower jump height
            player.on_ground = false;
            self.pending_sounds.push(SoundEffect::Jump);
        }

        // Clamp horizontal velocity (faster max walking speed)
        player.vel_x = player.vel_x.max(-4.0).min(4.0);  // Increased from 2.8 for faster walking
    }

    fn update_physics(&mut self) {
        const GRAVITY: f32 = 0.05;  // Very slow gravity for floaty jumps
        const MAX_FALL_SPEED: f32 = 2.0;  // Very slow terminal velocity

        // Get player position for hexagnome AI
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
                EntityType::Player | EntityType::Hexagnome | EntityType::BloodGoblin => {
                    // Handle death animation for player
                    if entity.is_dying {
                        entity.death_timer += 1.0;
                        entity.death_flash_timer += 1.0;
                        
                        // Death animation lasts 180 frames (3 seconds) to show falling through floor
                        if entity.death_timer >= 180.0 {
                            // Death animation complete - reset game
                            self.game_state = GameState::Intro;
                            self.lives = 3; // Reset lives for next game
                            self.init_world(); // Reset world
                            return; // Exit early to avoid processing the rest
                        }
                        
                        // Apply death physics (falling backward)
                        entity.vel_y += GRAVITY * 1.5; // Slightly faster fall when dying
                        entity.x += entity.vel_x;
                        entity.y += entity.vel_y;
                        
                        // No friction when dying (ragdoll effect)
                        entity.x = entity.x.max(0.0).min(self.world_width - entity.width);
                    } else {
                        // Special hopping behavior for BloodGoblin
                        if entity.entity_type == EntityType::BloodGoblin {
                            entity.animation_timer += 1.0;
                            
                            // Hop every 120 frames (2 seconds at 60fps) - slower hopping
                            if entity.on_ground && entity.animation_timer >= 120.0 {
                                entity.vel_y = -2.5; // Lower hop than before (was -4.5)
                                entity.on_ground = false;
                                entity.animation_timer = 0.0;
                                
                                // Add random horizontal movement (left to right)
                                let hop_direction = if (entity.animation_frame % 4) < 2 { -0.8 } else { 0.8 };
                                entity.vel_x = hop_direction;
                                
                                // Increment animation frame for next hop direction
                                entity.animation_frame = (entity.animation_frame + 1) % 4;
                            }
                        }
                        
                        // Normal physics
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
                            if entity.entity_type == EntityType::BloodGoblin {
                                entity.vel_x *= 0.85;  // Slightly less friction for blood goblin to allow sliding
                            } else {
                                entity.vel_x *= 0.75;  // More friction on ground for player/hexagnomes
                            }
                        } else {
                            entity.vel_x *= 0.985;  // Reduced air resistance for better air control
                        }

                        // Keep within world bounds
                        entity.x = entity.x.max(0.0).min(self.world_width - entity.width);
                    }
                }
                EntityType::Shuriken => {
                    // Shuriken physics
                    entity.x += entity.vel_x;
                    entity.y += entity.vel_y;
                    entity.vel_y += 0.005; // Minimal gravity to maintain range at slow speed

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

        // Enemy collision detection
        self.check_enemy_collisions();

        // Collectible collision detection
        self.check_collectible_collisions();

        // Ground collision - simple flat ground at Y=200
        for entity in &mut self.entities {
            if matches!(entity.entity_type, EntityType::Player | EntityType::Hexagnome | EntityType::BloodGoblin) {
                // Let dying players fall through the floor
                if entity.is_dying {
                    continue; // Skip ground collision for dying entities
                }
                
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
            if !matches!(entity.entity_type, EntityType::Player | EntityType::Hexagnome) {
                continue;
            }

            // Let dying players fall through platforms
            if entity.is_dying {
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

    fn check_enemy_collisions(&mut self) {
        if self.player_id >= self.entities.len() {
            return;
        }

        // Update invulnerability timer
        if self.invulnerability_timer > 0.0 {
            self.invulnerability_timer -= 1.0;
        }

        let player = &self.entities[self.player_id];
        
        // Don't check collisions if player is dying
        if player.is_dying {
            return;
        }
        
        let player_bounds = (player.x, player.y, player.x + player.width, player.y + player.height);

        let mut hit_detected = false;
        let mut hit_shuriken_indices = Vec::new();

        // Check collisions with hexagnomes and shuriken (with spatial optimization)
        for (i, entity) in self.entities.iter().enumerate() {
            if !entity.active || entity.entity_type == EntityType::Player || entity.entity_type == EntityType::Platform || entity.entity_type == EntityType::Collectible {
                continue;
            }

            // Spatial optimization: only check collision if entities are reasonably close
            let player_center_x = player_bounds.0 + (player_bounds.2 - player_bounds.0) / 2.0;
            let player_center_y = player_bounds.1 + (player_bounds.3 - player_bounds.1) / 2.0;
            let entity_center_x = entity.x + entity.width / 2.0;
            let entity_center_y = entity.y + entity.height / 2.0;
            
            let distance_squared = (player_center_x - entity_center_x).powi(2) + (player_center_y - entity_center_y).powi(2);
            
            // Only check detailed collision if within reasonable distance (80 pixels)
            if distance_squared > 80.0 * 80.0 {
                continue;
            }

            // Check if enemy/projectile overlaps with player (bounding box collision)
            if entity.x < player_bounds.2 &&
               entity.x + entity.width > player_bounds.0 &&
               entity.y < player_bounds.3 &&
               entity.y + entity.height > player_bounds.1 {

                // Only damage if not invulnerable
                if self.invulnerability_timer <= 0.0 {
                    hit_detected = true;
                    
                    // Mark shuriken for deactivation
                    if entity.entity_type == EntityType::Shuriken {
                        hit_shuriken_indices.push(i);
                    }
                }
            }
        }

        // Apply damage and effects after the loop
        if hit_detected {
            self.take_damage();
            // Set invulnerability period (180 frames = 3 seconds at 60fps)
            self.invulnerability_timer = 180.0;
        }

        // Deactivate hit shuriken
        for index in hit_shuriken_indices {
            self.entities[index].active = false;
        }
    }

    fn check_collectible_collisions(&mut self) {
        if self.player_id >= self.entities.len() {
            return;
        }

        let player = &self.entities[self.player_id];
        
        // Don't check collisions if player is dying
        if player.is_dying {
            return;
        }
        
        let player_bounds = (player.x, player.y, player.x + player.width, player.y + player.height);
        let mut collected_indices = Vec::new();

        // Check collisions with collectibles (hamberries) - with spatial optimization
        for (i, entity) in self.entities.iter().enumerate() {
            if !entity.active || entity.entity_type != EntityType::Collectible {
                continue;
            }

            // Spatial optimization: only check collision if reasonably close
            let player_center_x = player_bounds.0 + (player_bounds.2 - player_bounds.0) / 2.0;
            let player_center_y = player_bounds.1 + (player_bounds.3 - player_bounds.1) / 2.0;
            let entity_center_x = entity.x + entity.width / 2.0;
            let entity_center_y = entity.y + entity.height / 2.0;
            
            let distance_squared = (player_center_x - entity_center_x).powi(2) + (player_center_y - entity_center_y).powi(2);
            
            // Only check detailed collision if within collection distance (40 pixels)
            if distance_squared > 40.0 * 40.0 {
                continue;
            }

            // Check if collectible overlaps with player (bounding box collision)
            if entity.x < player_bounds.2 &&
               entity.x + entity.width > player_bounds.0 &&
               entity.y < player_bounds.3 &&
               entity.y + entity.height > player_bounds.1 {
                
                // Mark for collection
                collected_indices.push(i);
            }
        }

        // Collect hamberries and increase score
        for index in collected_indices {
            self.entities[index].active = false;
            self.score += 10; // 10 points per hamberry
            self.pending_sounds.push(SoundEffect::Collect);
        }
    }

    fn take_damage(&mut self) {
        if self.lives > 0 {
            self.lives -= 1;
            
            if self.lives == 0 {
                // Start death animation
                if self.player_id < self.entities.len() {
                    let player = &mut self.entities[self.player_id];
                    player.is_dying = true;
                    player.death_timer = 0.0;
                    player.death_flash_timer = 0.0;
                    player.vel_x = -1.5; // Fall backward slightly faster
                    player.vel_y = -8.0; // Big upward jump before falling through floor
                    player.on_ground = false; // Make sure he's airborne
                }
                self.pending_sounds.push(SoundEffect::Death);
            } else {
                // Just took damage, not dying yet
                self.pending_sounds.push(SoundEffect::EnemyHit);
            }
        }
    }

    fn update_hexagnome_spawning(&mut self) {
        // Count existing hexagnomes
        let hexagnome_count = self.entities.iter().filter(|e| e.entity_type == EntityType::Hexagnome && e.active).count();
        
        // Only spawn if we have fewer than 3 hexagnomes and it's time to spawn
        if hexagnome_count < 3 && self.frame_count % 600 == 0 {  // Limit to 3 hexagnomes max
            let spawn_x = if self.entities.len() % 2 == 0 { 0.0 } else { self.world_width - 20.0 };
            let hexagnome = Entity::new(EntityType::Hexagnome, spawn_x, 100.0, 3); // sprite ID 3 for hexagnome
            self.entities.push(hexagnome);
        }

        // Hexagnome AI and shuriken throwing
        if let Some((player_x, player_y)) = self.get_player_position() {
            for entity in &mut self.entities {
                if entity.entity_type == EntityType::Hexagnome && entity.active {
                    let dx = player_x - entity.x;
                    let distance = dx.abs();

                    if distance > 50.0 {
                        // Move toward player when far away
                        if dx > 0.0 {
                            entity.vel_x += 0.05;
                            entity.facing_left = true; // Face right toward player (flipped)
                        } else {
                            entity.vel_x -= 0.05;
                            entity.facing_left = false; // Face left toward player (flipped)
                        }
                        entity.vel_x = entity.vel_x.max(-1.0).min(1.0);
                    } else if distance < 30.0 {
                        // Change direction when too close - move away from player
                        if dx > 0.0 {
                            entity.vel_x -= 0.1; // Move left (away from player on right)
                            entity.facing_left = false; // Face left while retreating (flipped)
                        } else {
                            entity.vel_x += 0.1; // Move right (away from player on left)
                            entity.facing_left = true; // Face right while retreating (flipped)
                        }
                        entity.vel_x = entity.vel_x.max(-1.2).min(1.2); // Slightly faster when retreating
                    }

                    // Throw shuriken rarely
                    if distance < 200.0 && distance > 50.0 && self.frame_count % 300 == 0 {
                        let mut shuriken = Entity::new(EntityType::Shuriken, entity.x + 10.0, entity.y + 10.0, 4); // sprite ID 4 for shuriken
                        let dy = player_y - entity.y;
                        let speed = 0.6; // Really slow speed
                        let distance = (dx * dx + dy * dy).sqrt();

                        if distance > 0.0 {
                            // More horizontal trajectory: reduce arc height
                            shuriken.vel_x = (dx / distance) * speed * 1.2; // 20% faster horizontally
                            shuriken.vel_y = (dy / distance) * speed * 0.15 - 0.3; // Reduced upward bias for lower arc
                        }

                        self.pending_shuriken.push(shuriken);
                        self.pending_sounds.push(SoundEffect::ShurikenThrow);
                    }
                }
            }
        }
    }

    fn check_blood_goblin_spawn(&mut self) {
        // Check if player is near the end of the level (world_width - 300)
        if let Some((player_x, player_y)) = self.get_player_position() {
            let level_end_trigger = self.world_width - 300.0; // Trigger 300 pixels before end
            
            // Only spawn if player has reached the end and no blood goblin exists yet
            if player_x >= level_end_trigger {
                let blood_goblin_exists = self.entities.iter().any(|e| e.entity_type == EntityType::BloodGoblin && e.active);
                
                if !blood_goblin_exists {
                    // Spawn blood goblin at the end of the level, on the ground
                    let blood_goblin_x = self.world_width - 100.0; // Near the very end
                    let blood_goblin_y = 168.0 - 38.0; // Ground level - sprite height (38)
                    let mut blood_goblin = Entity::new(EntityType::BloodGoblin, blood_goblin_x, blood_goblin_y, 7); // sprite ID 7
                    
                    // Set initial hopping state
                    blood_goblin.vel_y = -3.0; // Start with an upward hop
                    blood_goblin.animation_timer = 0.0;
                    
                    self.entities.push(blood_goblin);
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
        js_sys::Reflect::set(&obj, &"facing_left".into(), &entity.facing_left.into()).unwrap();

        Some(obj)
    }

    fn spawn_hamberries(&mut self) {
        // Spawn 20 hamberries scattered throughout the level
        let hamberry_positions = [
            (120.0, 180.0),   // Ground level
            (300.0, 160.0),   // Near first platform
            (230.0, 150.0),   // On first platform
            (450.0, 120.0),   // On second platform
            (550.0, 170.0),   // Ground level
            (630.0, 90.0),    // On third platform
            (750.0, 180.0),   // Ground level
            (830.0, 110.0),   // On fourth platform
            (950.0, 170.0),   // Ground level
            (1030.0, 80.0),   // On fifth platform
            (1150.0, 180.0),  // Ground level
            (1230.0, 70.0),   // On sixth platform
            (1350.0, 180.0),  // Ground level
            (1430.0, 100.0),  // On seventh platform
            (1550.0, 170.0),  // Ground level
            (1630.0, 60.0),   // On eighth platform
            (1750.0, 180.0),  // Ground level
            (1830.0, 90.0),   // On ninth platform
            (1900.0, 170.0),  // Ground level
            (1950.0, 160.0),  // Near end
        ];

        for (x, y) in hamberry_positions.iter() {
            self.entities.push(Entity::new(EntityType::Collectible, *x, *y, 6));
        }
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

    pub fn get_lives(&self) -> u32 {
        self.lives
    }

    pub fn get_score(&self) -> u32 {
        self.score
    }

    pub fn is_player_dying(&self) -> bool {
        if self.player_id < self.entities.len() {
            self.entities[self.player_id].is_dying
        } else {
            false
        }
    }

    pub fn get_player_death_flash(&self) -> bool {
        if self.player_id < self.entities.len() {
            let player = &self.entities[self.player_id];
            if player.is_dying {
                // Flash every 8 frames (fast flashing)
                (player.death_flash_timer as u32 / 8) % 2 == 0
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn is_player_invulnerable(&self) -> bool {
        self.invulnerability_timer > 0.0
    }

    pub fn get_player_invul_flash(&self) -> bool {
        if self.invulnerability_timer > 0.0 {
            // Flash effect - alternate every 8 frames for slower flashing
            ((self.invulnerability_timer as u32) / 8) % 2 == 0
        } else {
            false
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

// Piano key data for visualization
#[derive(Clone)]
pub struct PianoKey {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub is_black: bool,
    pub is_pressed: bool,
    pub keyboard_key: char,  // The ZSXDCVGBHNJM key that triggers this note
    pub note: u32,          // MIDI note number
}

// Z-Synth cartridge - A synthesizer application
#[wasm_bindgen]
pub struct ZSynthCartridge {
    // Key mappings for ZSXDCVGBHNJM -> C2+ notes
    key_to_note: HashMap<char, u32>,
    // Currently pressed keys and their corresponding notes
    active_notes: HashMap<char, u32>,
    // Frame counter for animations
    frame_count: u64,
    // Audio-related state
    pending_note_on: Vec<u32>,
    pending_note_off: Vec<u32>,
    // Piano key visualization
    piano_keys: Vec<PianoKey>,
    // Arpeggio state
    arpeggio_timers: HashMap<char, f32>,  // Track how long each key has been held
    arpeggio_patterns: HashMap<char, Vec<i32>>,  // Minor arpeggio patterns for each key
    current_arpeggio_notes: HashMap<char, u32>,  // Currently playing arpeggio note for each key
    arpeggio_step: HashMap<char, usize>,  // Current step in arpeggio pattern for each key
    // MIDI state
    midi_active_notes: HashMap<u32, bool>,  // Track MIDI notes that are currently on
    midi_arpeggio_timers: HashMap<u32, f32>,  // Arpeggio timers for MIDI notes
    midi_arpeggio_step: HashMap<u32, usize>,  // Arpeggio steps for MIDI notes
    midi_current_arpeggio_notes: HashMap<u32, u32>,  // Current arpeggio note for each MIDI note
}

#[wasm_bindgen]
impl ZSynthCartridge {
    #[wasm_bindgen(constructor)]
    pub fn new() -> ZSynthCartridge {
        let mut key_to_note = HashMap::new();
        let mut piano_keys = Vec::new();
        
        // Map ZSXDCVGBHNJM to chromatic notes starting from C2 (MIDI 36)
        let keys = ['z', 's', 'x', 'd', 'c', 'v', 'g', 'b', 'h', 'n', 'j', 'm'];
        
        // MIDI note 36 = C2, create a chromatic octave: C, C#, D, D#, E, F, F#, G, G#, A, A#, B
        let note_names = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];
        let black_keys = [false, true, false, true, false, false, true, false, true, false, true, false];
        
        // Piano layout: white keys are wider, black keys are narrower and offset
        let mut white_key_index = 0;
        let white_key_width = 25.0;  // Smaller to fit screen
        let black_key_width = 15.0;  // Smaller to fit screen
        let white_key_height = 80.0; // Shorter to fit screen
        let black_key_height = 50.0; // Shorter to fit screen
        let keyboard_start_x = 10.0;    // Center horizontally (320px screen - 300px keyboard = 20px / 2 = 10px)
        let keyboard_y = 100.0;      // Move up a bit for better centering
        
        for (i, &key) in keys.iter().enumerate() {
            let note = 36 + i as u32;
            let is_black = black_keys[i];
            key_to_note.insert(key, note);
            
            let (x, y, width, height) = if is_black {
                // Black keys are positioned between white keys
                let prev_white_x = keyboard_start_x + (white_key_index as f32 - 0.5) * white_key_width;
                (prev_white_x - black_key_width / 2.0, keyboard_y, black_key_width, black_key_height)
            } else {
                // White keys are positioned sequentially
                let x = keyboard_start_x + white_key_index as f32 * white_key_width;
                white_key_index += 1;
                (x, keyboard_y, white_key_width, white_key_height)
            };
            
            piano_keys.push(PianoKey {
                x,
                y,
                width,
                height,
                is_black,
                is_pressed: false,
                keyboard_key: key,
                note,
            });
        }
        
        // Create minor arpeggio patterns for each key
        // Minor triad pattern: root, minor third, fifth, octave, fifth, minor third (and repeat)
        let mut arpeggio_patterns = HashMap::new();
        for (i, &key) in keys.iter().enumerate() {
            let root_note = 36 + i as u32;
            // Minor arpeggio: root, +3 semitones (minor third), +7 semitones (fifth), +12 (octave)
            let pattern = vec![0, 3, 7, 12, 7, 3]; // Relative to root note
            arpeggio_patterns.insert(key, pattern);
        }

        ZSynthCartridge {
            key_to_note,
            active_notes: HashMap::new(),
            frame_count: 0,
            pending_note_on: Vec::new(),
            pending_note_off: Vec::new(),
            piano_keys,
            arpeggio_timers: HashMap::new(),
            arpeggio_patterns,
            current_arpeggio_notes: HashMap::new(),
            arpeggio_step: HashMap::new(),
            midi_active_notes: HashMap::new(),
            midi_arpeggio_timers: HashMap::new(),
            midi_arpeggio_step: HashMap::new(),
            midi_current_arpeggio_notes: HashMap::new(),
        }
    }

    pub fn handle_key_down(&mut self, key: char) {
        let key_lower = key.to_ascii_lowercase();
        if let Some(&note) = self.key_to_note.get(&key_lower) {
            if !self.active_notes.contains_key(&key_lower) {
                self.active_notes.insert(key_lower, note);
                
                // Initialize arpeggio state for this key
                self.arpeggio_timers.insert(key_lower, 0.0);
                self.arpeggio_step.insert(key_lower, 0);
                
                // Play the first note of the arpeggio (root note)
                self.pending_note_on.push(note);
                self.current_arpeggio_notes.insert(key_lower, note);
                
                // Update visual piano key
                for piano_key in &mut self.piano_keys {
                    if piano_key.keyboard_key == key_lower {
                        piano_key.is_pressed = true;
                        break;
                    }
                }
            }
        }
    }

    pub fn handle_key_up(&mut self, key: char) {
        let key_lower = key.to_ascii_lowercase();
        if let Some(_note) = self.active_notes.remove(&key_lower) {
            // Stop the current arpeggio note
            if let Some(current_arp_note) = self.current_arpeggio_notes.remove(&key_lower) {
                self.pending_note_off.push(current_arp_note);
            }
            
            // Clean up arpeggio state
            self.arpeggio_timers.remove(&key_lower);
            self.arpeggio_step.remove(&key_lower);
            
            // Update visual piano key
            for piano_key in &mut self.piano_keys {
                if piano_key.keyboard_key == key_lower {
                    piano_key.is_pressed = false;
                    break;
                }
            }
        }
    }

    pub fn update_synth(&mut self) {
        self.frame_count += 1;
        
        // Update arpeggio timers and advance notes for held keys
        let dt = 1.0 / 60.0; // Assuming 60 FPS
        let arpeggio_speed = 0.3; // Time between arpeggio notes in seconds
        
        let mut keys_to_update: Vec<char> = self.arpeggio_timers.keys().cloned().collect();
        
        for key in keys_to_update {
            if let Some(timer) = self.arpeggio_timers.get_mut(&key) {
                *timer += dt;
                
                // Check if it's time to advance to the next arpeggio note
                if *timer >= arpeggio_speed {
                    *timer = 0.0; // Reset timer
                    
                    // Stop current arpeggio note
                    if let Some(current_note) = self.current_arpeggio_notes.get(&key) {
                        self.pending_note_off.push(*current_note);
                    }
                    
                    // Advance to next step in arpeggio pattern
                    if let Some(step) = self.arpeggio_step.get_mut(&key) {
                        if let Some(pattern) = self.arpeggio_patterns.get(&key) {
                            *step = (*step + 1) % pattern.len();
                            
                            // Calculate and play the next arpeggio note
                            if let Some(&root_note) = self.key_to_note.get(&key) {
                                let pattern_offset = pattern[*step];
                                let arpeggio_note = (root_note as i32 + pattern_offset) as u32;
                                
                                self.pending_note_on.push(arpeggio_note);
                                self.current_arpeggio_notes.insert(key, arpeggio_note);
                            }
                        }
                    }
                }
            }
        }

        // Update MIDI arpeggio timers and advance notes for held MIDI notes
        let midi_keys_to_update: Vec<u32> = self.midi_arpeggio_timers.keys().cloned().collect();
        
        for midi_note in midi_keys_to_update {
            if let Some(timer) = self.midi_arpeggio_timers.get_mut(&midi_note) {
                *timer += dt;
                
                // Check if it's time to advance to the next arpeggio note
                if *timer >= arpeggio_speed {
                    *timer = 0.0; // Reset timer
                    
                    // Stop current arpeggio note
                    if let Some(current_note) = self.midi_current_arpeggio_notes.get(&midi_note) {
                        self.pending_note_off.push(*current_note);
                    }
                    
                    // Advance to next step in arpeggio pattern
                    if let Some(step) = self.midi_arpeggio_step.get_mut(&midi_note) {
                        let pattern = vec![0, 3, 7, 12, 7, 3]; // Same minor arpeggio pattern
                        *step = (*step + 1) % pattern.len();
                        
                        let pattern_offset = pattern[*step];
                        let arpeggio_note = (midi_note as i32 + pattern_offset) as u32;
                        
                        self.pending_note_on.push(arpeggio_note);
                        self.midi_current_arpeggio_notes.insert(midi_note, arpeggio_note);
                    }
                }
            }
        }
    }

    // MIDI note handlers
    pub fn handle_midi_note_on(&mut self, note: u32) {
        if !self.midi_active_notes.contains_key(&note) {
            self.midi_active_notes.insert(note, true);
            
            // Initialize arpeggio state for this MIDI note
            self.midi_arpeggio_timers.insert(note, 0.0);
            self.midi_arpeggio_step.insert(note, 0);
            
            // Play the first note of the arpeggio (root note)
            self.pending_note_on.push(note);
            self.midi_current_arpeggio_notes.insert(note, note);
        }
    }

    pub fn handle_midi_note_off(&mut self, note: u32) {
        if self.midi_active_notes.remove(&note).is_some() {
            // Stop the current arpeggio note
            if let Some(current_arp_note) = self.midi_current_arpeggio_notes.remove(&note) {
                self.pending_note_off.push(current_arp_note);
            }
            
            // Clean up arpeggio state
            self.midi_arpeggio_timers.remove(&note);
            self.midi_arpeggio_step.remove(&note);
        }
    }

    // Get active MIDI notes for display
    pub fn get_active_midi_notes(&self) -> Vec<u32> {
        self.midi_active_notes.keys().cloned().collect()
    }

    pub fn get_active_midi_note_count(&self) -> usize {
        self.midi_active_notes.len()
    }

    // Get pending note on events for audio processing
    pub fn get_pending_note_on(&self) -> Vec<u32> {
        self.pending_note_on.clone()
    }

    // Get pending note off events for audio processing
    pub fn get_pending_note_off(&self) -> Vec<u32> {
        self.pending_note_off.clone()
    }

    // Clear pending note events after processing
    pub fn clear_pending_notes(&mut self) {
        self.pending_note_on.clear();
        self.pending_note_off.clear();
    }

    // Get currently active notes for display
    pub fn get_active_note_count(&self) -> usize {
        self.active_notes.len()
    }

    pub fn get_active_notes_info(&self) -> String {
        let mut info = String::new();
        for (key, note) in &self.active_notes {
            if !info.is_empty() {
                info.push_str(", ");
            }
            info.push_str(&format!("{}:{}", key.to_uppercase(), note));
        }
        info
    }

    // Get frame count for animations
    pub fn get_frame_count(&self) -> u64 {
        self.frame_count
    }

    // Piano visualization methods
    pub fn get_piano_key_count(&self) -> usize {
        self.piano_keys.len()
    }

    pub fn get_piano_key_data(&self, index: usize) -> Option<js_sys::Object> {
        if index >= self.piano_keys.len() {
            return None;
        }

        let key = &self.piano_keys[index];
        let obj = js_sys::Object::new();

        js_sys::Reflect::set(&obj, &"x".into(), &key.x.into()).unwrap();
        js_sys::Reflect::set(&obj, &"y".into(), &key.y.into()).unwrap();
        js_sys::Reflect::set(&obj, &"width".into(), &key.width.into()).unwrap();
        js_sys::Reflect::set(&obj, &"height".into(), &key.height.into()).unwrap();
        js_sys::Reflect::set(&obj, &"is_black".into(), &key.is_black.into()).unwrap();
        js_sys::Reflect::set(&obj, &"is_pressed".into(), &key.is_pressed.into()).unwrap();
        js_sys::Reflect::set(&obj, &"keyboard_key".into(), &key.keyboard_key.to_string().into()).unwrap();
        js_sys::Reflect::set(&obj, &"note".into(), &key.note.into()).unwrap();

        Some(obj)
    }
}