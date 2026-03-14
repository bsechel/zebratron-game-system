#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "wasm")]
use web_sys::console;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

macro_rules! log_internal {
    ( $( $t:tt )* ) => {
        #[cfg(feature = "wasm")]
        console::log_1(&format!( $( $t )* ).into());
        #[cfg(not(feature = "wasm"))]
        println!( $( $t )* );
    }
}

#[derive(Debug, Clone)]
pub struct TileData {
    pub pixels: Vec<u32>, // RGBA pixel data
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone)]
pub struct TileSheet {
    pub tiles: HashMap<u16, TileData>,
    pub tile_width: u32,
    pub tile_height: u32,
    pub sheet_width: u32,
    pub sheet_height: u32,
}

impl TileSheet {
    pub fn new(tile_width: u32, tile_height: u32) -> Self {
        Self {
            tiles: HashMap::new(),
            tile_width,
            tile_height,
            sheet_width: 0,
            sheet_height: 0,
        }
    }

    pub fn load_from_png_data(&mut self, png_data: &[u8], sheet_width: u32, sheet_height: u32) -> Result<(), String> {
        // For now, we'll expect the JavaScript side to decode the PNG and send us RGBA data
        // This is a simplified approach - in a real implementation you'd use a PNG decoder crate
        if png_data.len() != (sheet_width * sheet_height * 4) as usize {
            return Err(format!("Invalid PNG data size. Expected {} bytes, got {}", 
                sheet_width * sheet_height * 4, png_data.len()));
        }

        self.sheet_width = sheet_width;
        self.sheet_height = sheet_height;
        
        // Calculate how many tiles fit in this sheet
        let tiles_x = sheet_width / self.tile_width;
        let tiles_y = sheet_height / self.tile_height;
        
        log_internal!("Loading tilesheet: {}x{} pixels, {}x{} tiles", 
            sheet_width, sheet_height, tiles_x, tiles_y);

        // Extract each tile
        for tile_y in 0..tiles_y {
            for tile_x in 0..tiles_x {
                let tile_id = (tile_y * tiles_x + tile_x) as u16;
                let tile_data = self.extract_tile(png_data, sheet_width, tile_x, tile_y)?;
                self.tiles.insert(tile_id, tile_data);
            }
        }

        log_internal!("Loaded {} tiles from tilesheet", self.tiles.len());
        Ok(())
    }

    fn extract_tile(&self, sheet_data: &[u8], sheet_width: u32, tile_x: u32, tile_y: u32) -> Result<TileData, String> {
        let mut pixels = Vec::with_capacity((self.tile_width * self.tile_height) as usize);
        
        let start_x = tile_x * self.tile_width;
        let start_y = tile_y * self.tile_height;
        
        for y in 0..self.tile_height {
            for x in 0..self.tile_width {
                let sheet_x = start_x + x;
                let sheet_y = start_y + y;
                let sheet_index = ((sheet_y * sheet_width + sheet_x) * 4) as usize;
                
                if sheet_index + 3 >= sheet_data.len() {
                    return Err("Index out of bounds while extracting tile".to_string());
                }
                
                // Convert RGBA bytes to single u32
                let r = sheet_data[sheet_index] as u32;
                let g = sheet_data[sheet_index + 1] as u32;
                let b = sheet_data[sheet_index + 2] as u32;
                let a = sheet_data[sheet_index + 3] as u32;
                
                let pixel = (a << 24) | (r << 16) | (g << 8) | b;
                pixels.push(pixel);
            }
        }
        
        Ok(TileData {
            pixels,
            width: self.tile_width,
            height: self.tile_height,
        })
    }

    pub fn get_tile(&self, tile_id: u16) -> Option<&TileData> {
        self.tiles.get(&tile_id)
    }

    pub fn get_tile_count(&self) -> usize {
        self.tiles.len()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct LevelSize {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Spawn {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TileSize {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LevelData {
    pub name: String,
    pub description: Option<String>,
    pub tilesheet: String,
    pub tile_size: TileSize,
    pub level_size: LevelSize,
    pub spawn: Spawn,
    pub tile_collisions: HashMap<String, String>,
    pub tiles: Vec<Vec<u16>>,
}

impl LevelData {
    pub fn new(name: &str, width: u32, height: u32) -> Self {
        Self {
            name: name.to_string(),
            description: None,
            tilesheet: "embedded".to_string(),
            tile_size: TileSize { width: 16, height: 16 },
            level_size: LevelSize { width, height },
            spawn: Spawn { x: 32, y: 192 },
            tile_collisions: HashMap::new(),
            tiles: vec![vec![0; width as usize]; height as usize],
        }
    }
    
    pub fn with_tiles(mut self, tiles: Vec<Vec<u16>>) -> Self {
        self.tiles = tiles;
        self
    }
    
    pub fn with_collision_map(mut self, collisions: HashMap<String, String>) -> Self {
        self.tile_collisions = collisions;
        self
    }
    
    pub fn with_spawn(mut self, x: u32, y: u32) -> Self {
        self.spawn = Spawn { x, y };
        self
    }
}

#[derive(Debug, Clone)]
pub struct Level {
    pub width: u32,
    pub height: u32,
    pub tile_data: Vec<Vec<u16>>, // 2D array of tile IDs
    pub collision_map: Vec<Vec<CollisionType>>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CollisionType {
    None,
    Solid,
    OneWayPlatform,
    Spike,
    Ladder,
    Water,
}

impl Default for CollisionType {
    fn default() -> Self {
        CollisionType::None
    }
}

impl Level {
    pub fn new(width: u32, height: u32) -> Self {
        let tile_data = vec![vec![0; width as usize]; height as usize];
        let collision_map = vec![vec![CollisionType::None; width as usize]; height as usize];
        
        Self {
            width,
            height,
            tile_data,
            collision_map,
        }
    }

    pub fn load_from_data(&mut self, level_data: &LevelData) -> Result<(), String> {
        log_internal!("Loading level: {}", level_data.name);
        
        // Update dimensions
        self.width = level_data.level_size.width;
        self.height = level_data.level_size.height;
        
        // Resize arrays
        self.tile_data = vec![vec![0; self.width as usize]; self.height as usize];
        self.collision_map = vec![vec![CollisionType::None; self.width as usize]; self.height as usize];
        
        // Load tile data
        if level_data.tiles.len() != self.height as usize {
            return Err(format!("Tile data height mismatch: expected {}, got {}", 
                self.height, level_data.tiles.len()));
        }
        
        for (y, row) in level_data.tiles.iter().enumerate() {
            if row.len() != self.width as usize {
                return Err(format!("Tile data width mismatch at row {}: expected {}, got {}", 
                    y, self.width, row.len()));
            }
            
            for (x, &tile_id) in row.iter().enumerate() {
                self.tile_data[y][x] = tile_id;
                
                // Set collision based on tile collisions map
                let collision = level_data.tile_collisions.get(&tile_id.to_string())
                    .map(|s| match s.as_str() {
                        "solid" => CollisionType::Solid,
                        "water" => CollisionType::Water,
                        "spike" => CollisionType::Spike,
                        "ladder" => CollisionType::Ladder,
                        "oneway" => CollisionType::OneWayPlatform,
                        _ => CollisionType::None,
                    })
                    .unwrap_or(CollisionType::None);
                
                self.collision_map[y][x] = collision;
            }
        }
        
        log_internal!("Level loaded: {}x{} tiles", self.width, self.height);
        Ok(())
    }

    pub fn get_tile_at(&self, x: u32, y: u32) -> Option<u16> {
        if x < self.width && y < self.height {
            Some(self.tile_data[y as usize][x as usize])
        } else {
            None
        }
    }

    pub fn get_collision_at(&self, x: u32, y: u32) -> CollisionType {
        if x < self.width && y < self.height {
            self.collision_map[y as usize][x as usize]
        } else {
            CollisionType::Solid // Treat out-of-bounds as solid
        }
    }

    pub fn set_tile_at(&mut self, x: u32, y: u32, tile_id: u16, collision: CollisionType) {
        if x < self.width && y < self.height {
            self.tile_data[y as usize][x as usize] = tile_id;
            self.collision_map[y as usize][x as usize] = collision;
        }
    }
}

// WASM bindings for JavaScript interface
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct TileSystem {
    tilesheet: TileSheet,
    level: Level,
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl TileSystem {
    #[cfg_attr(feature = "wasm", wasm_bindgen(constructor))]
    pub fn new(tile_width: u32, tile_height: u32) -> TileSystem {
        log_internal!("Creating new TileSystem");
        
        TileSystem {
            tilesheet: TileSheet::new(tile_width, tile_height),
            level: Level::new(0, 0),
        }
    }

    #[cfg_attr(feature = "wasm", wasm_bindgen)]
    pub fn load_tilesheet(&mut self, png_data: &[u8], sheet_width: u32, sheet_height: u32) -> bool {
        match self.tilesheet.load_from_png_data(png_data, sheet_width, sheet_height) {
            Ok(_) => {
                log_internal!("Tilesheet loaded successfully");
                true
            }
            Err(e) => {
                log_internal!("Failed to load tilesheet: {}", e);
                false
            }
        }
    }

    #[cfg_attr(feature = "wasm", wasm_bindgen)]
    pub fn create_level(&mut self, width: u32, height: u32) {
        self.level = Level::new(width, height);
        log_internal!("Created level: {}x{}", width, height);
    }

    #[cfg_attr(feature = "wasm", wasm_bindgen)]
    pub fn load_level_json(&mut self, json_data: &str) -> bool {
        // Parse JSON into LevelData first
        match serde_json::from_str::<LevelData>(json_data) {
            Ok(level_data) => {
                match self.level.load_from_data(&level_data) {
                    Ok(_) => {
                        log_internal!("Level JSON loaded successfully");
                        true
                    }
                    Err(e) => {
                        log_internal!("Failed to load level data: {}", e);
                        false
                    }
                }
            }
            Err(e) => {
                log_internal!("Failed to parse level JSON: {}", e);
                false
            }
        }
    }

    #[cfg_attr(feature = "wasm", wasm_bindgen)]
    pub fn get_tile_count(&self) -> usize {
        self.tilesheet.get_tile_count()
    }

    #[cfg_attr(feature = "wasm", wasm_bindgen)]
    pub fn get_tile_at(&self, x: u32, y: u32) -> u16 {
        self.level.get_tile_at(x, y).unwrap_or(0)
    }

    #[cfg_attr(feature = "wasm", wasm_bindgen)]
    pub fn set_tile_at(&mut self, x: u32, y: u32, tile_id: u16) {
        // For now, assume solid collision for non-zero tiles
        let collision = if tile_id == 0 {
            CollisionType::None
        } else {
            CollisionType::Solid
        };
        
        self.level.set_tile_at(x, y, tile_id, collision);
    }

    // Get tile pixel data for rendering
    #[cfg_attr(feature = "wasm", wasm_bindgen)]
    pub fn get_tile_pixels(&self, tile_id: u16) -> Option<Vec<u32>> {
        self.tilesheet.get_tile(tile_id).map(|tile| tile.pixels.clone())
    }
}