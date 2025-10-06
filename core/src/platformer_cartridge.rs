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

pub struct PlatformerCartridge {
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
}

impl PlatformerCartridge {
    pub fn new() -> Self {
        // Create an elaborate level: 0=air, 1=solid block, 2=platform, 3=pitfall (deadly), 4=passage (leads down), 5=water, 6=swim-through
        let mut tiles = [[0u8; 200]; 15];
        
        // Ground at bottom (row 12-14) with varied terrain
        for row in 12..15 {
            for col in 0..200 {
                tiles[row][col] = 1;
            }
        }
        
        // Create pitfalls and passages in the ground
        // Pitfall 1: Early deadly challenge (columns 12-14)
        for row in 12..15 {
            for col in 12..15 {
                tiles[row][col] = 3; // Pitfall - kills player
            }
        }
        
        // Pitfall 2: Medium deadly challenge (columns 22-26)
        for row in 12..15 {
            for col in 22..27 {
                tiles[row][col] = 3; // Pitfall - kills player
            }
        }
        
        // Passage 1: Canyon passage to underground (columns 85-94)
        for row in 12..15 {
            for col in 85..95 {
                tiles[row][col] = 4; // Passage - leads down with camera follow
            }
        }
        
        // Passage 2: Underground tunnel entrance (columns 106-108)
        for row in 12..15 {
            for col in 106..109 {
                tiles[row][col] = 4; // Passage - leads to underground area
            }
        }
        
        // Pitfall 3: Mountain valley pitfall (columns 137-140)
        for row in 12..15 {
            for col in 137..141 {
                tiles[row][col] = 3; // Pitfall - kills player
            }
        }
        
        // Pitfall 4: Castle moat (columns 162-167)
        for row in 12..15 {
            for col in 162..168 {
                tiles[row][col] = 3; // Pitfall - kills player
            }
        }
        
        // Pitfall 5: Final deadly challenge (columns 185-188)
        for row in 12..15 {
            for col in 185..189 {
                tiles[row][col] = 3; // Pitfall - kills player
            }
        }
        
        // Create extensive terrain features across 200 tiles
        
        // Section 1: Starting area (0-20)
        for col in 0..8 { 
            tiles[11][col] = 1; // Raised starting platform
        }
        for col in 10..13 { tiles[10][col] = 1; }   // First jump
        for col in 15..18 { tiles[9][col] = 1; }    // Higher platform
        
        // Section 2: Stepping stones area (20-40)
        for col in 20..23 { tiles[8][col] = 1; }    // High platform
        for col in 25..28 { tiles[10][col] = 1; }   // Drop down
        for col in 30..33 { tiles[9][col] = 1; }    // Back up
        for col in 35..40 { tiles[7][col] = 1; }    // Very high platform
        
        // Section 3: Staircase area (40-60)
        tiles[11][42] = 1;
        tiles[10][43] = 1; tiles[11][43] = 1;
        tiles[9][44] = 1; tiles[10][44] = 1; tiles[11][44] = 1;
        tiles[8][45] = 1; tiles[9][45] = 1; tiles[10][45] = 1; tiles[11][45] = 1;
        tiles[7][46] = 1; tiles[8][46] = 1; tiles[9][46] = 1; tiles[10][46] = 1; tiles[11][46] = 1;
        for col in 47..55 { tiles[6][col] = 1; }    // Sky bridge
        
        // Section 4: Tower area (60-80)
        for col in 60..65 { 
            for row in 5..12 { tiles[row][col] = 1; } // Tall tower
        }
        for col in 67..70 { tiles[8][col] = 1; }    // Platform after tower
        for col in 72..75 { tiles[10][col] = 1; }   // Lower platform
        for col in 77..80 { tiles[7][col] = 1; }    // High platform
        
        // Section 5: Canyon area (80-100)
        for col in 80..85 { tiles[11][col] = 1; }   // Edge before canyon
        // Passage from 85-95 (canyon passage to underground)
        for col in 95..100 { tiles[11][col] = 1; }  // Other side of canyon
        
        // Section 6: Underground area (100-120)
        for col in 100..105 { tiles[9][col] = 1; }  // Down into tunnel
        for col in 105..115 { tiles[11][col] = 1; } // Tunnel floor
        for col in 115..120 { tiles[8][col] = 1; }  // Back up
        
        // Section 7: Mountain area (120-140)
        for col in 120..125 { tiles[10][col] = 1; }
        for col in 125..130 { tiles[8][col] = 1; }  // Peak
        for col in 130..135 { tiles[9][col] = 1; }  // Down slope
        for col in 135..140 { tiles[11][col] = 1; } // Valley floor
        
        // Section 8: Floating islands (140-160) - More spaced out and higher
        for col in 142..145 { tiles[7][col] = 1; }  // Island 1 (higher)
        for col in 149..152 { tiles[5][col] = 1; }  // Island 2 (much higher, more spacing)
        for col in 156..159 { tiles[6][col] = 1; }  // Island 3 (higher, more spacing)
        for col in 163..166 { tiles[8][col] = 1; }  // Island 4 (higher, more spacing)
        
        // Section 9: Castle approach (160-180)
        for col in 160..165 { tiles[11][col] = 1; } // Ground level
        for col in 165..170 { 
            for row in 8..12 { tiles[row][col] = 1; } // Castle wall
        }
        for col in 170..175 { tiles[7][col] = 1; }  // Castle ramparts
        for col in 175..180 { tiles[9][col] = 1; }  // Descent from castle
        
        // Section 10: Final area (180-200)
        for col in 180..185 { tiles[10][col] = 1; } // Final platforms
        for col in 187..192 { tiles[8][col] = 1; }  // Victory platform
        for col in 194..200 { 
            for row in 10..12 { tiles[row][col] = 1; } // End area
        }
        
        // Add some decorative elements and smaller platforms
        tiles[6][35] = 1; tiles[6][36] = 1; // Small floating platform
        tiles[5][47] = 1; tiles[5][48] = 1; // Tiny high platform
        tiles[9][90] = 1; // Single block in canyon
        tiles[7][110] = 1; tiles[7][111] = 1; // Tunnel ceiling decoration
        
        Self {
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
        }
    }
    
    pub fn update(&mut self, input: u8) {
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
}