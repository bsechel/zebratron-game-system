use image::{Rgba};
use std::collections::HashMap;
use std::env;
use std::fs;

// Master palette from ZebratronGameSystem PPU
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
    (0, 128, 128), (0, 160, 160), (0, 192, 192), (0, 224, 224),
    (0, 255, 255), (32, 255, 255), (64, 255, 255), (96, 255, 255),
    (128, 255, 255), (160, 255, 255), (192, 255, 255), (224, 255, 255),
    (0, 64, 128), (0, 96, 160), (32, 128, 192), (64, 160, 224),
    // Blues (80-95)
    (0, 0, 128), (0, 0, 160), (0, 0, 192), (0, 0, 224),
    (0, 0, 255), (32, 32, 255), (64, 64, 255), (96, 96, 255),
    (128, 128, 255), (160, 160, 255), (192, 192, 255), (224, 224, 255),
    (64, 0, 128), (96, 0, 160), (128, 32, 192), (160, 64, 224),
    // Purples/Magentas (96-111)
    (128, 0, 128), (160, 0, 160), (192, 0, 192), (224, 0, 224),
    (255, 0, 255), (255, 32, 255), (255, 64, 255), (255, 96, 255),
    (255, 128, 255), (255, 160, 255), (255, 192, 255), (255, 224, 255),
    (128, 0, 64), (160, 0, 96), (192, 32, 128), (224, 64, 160),
    // Special colors (112-127)
    (255, 192, 203), (255, 218, 185), (245, 245, 220), (255, 228, 196),
    (255, 105, 180), (255, 20, 147), (255, 69, 0), (255, 140, 0),
    (127, 255, 0), (0, 255, 127), (72, 61, 139), (106, 90, 205),
    (173, 216, 230), (135, 206, 235), (70, 130, 180), (25, 25, 112),
];

struct SpriteConverter {
    color_cache: HashMap<(u8, u8, u8), u8>,
}

impl SpriteConverter {
    fn new() -> Self {
        let mut color_cache = HashMap::new();
        
        // Pre-populate cache with exact palette matches
        for (index, &(r, g, b)) in MASTER_PALETTE.iter().enumerate() {
            color_cache.insert((r, g, b), index as u8);
        }
        
        Self { color_cache }
    }
    
    fn find_closest_palette_color(&mut self, r: u8, g: u8, b: u8) -> u8 {
        // Check cache first
        if let Some(&index) = self.color_cache.get(&(r, g, b)) {
            return index;
        }
        
        // Find closest color using Euclidean distance
        let mut best_index = 0;
        let mut best_distance = f32::MAX;
        
        for (index, &(pr, pg, pb)) in MASTER_PALETTE.iter().enumerate() {
            let dr = (r as f32) - (pr as f32);
            let dg = (g as f32) - (pg as f32);
            let db = (b as f32) - (pb as f32);
            let distance = (dr * dr + dg * dg + db * db).sqrt();
            
            if distance < best_distance {
                best_distance = distance;
                best_index = index;
            }
        }
        
        // Cache the result
        self.color_cache.insert((r, g, b), best_index as u8);
        best_index as u8
    }
    
    fn convert_png_to_sprite(&mut self, png_data: &[u8], sprite_name: &str) -> Result<String, Box<dyn std::error::Error>> {
        // Load PNG image
        let image = image::load_from_memory(png_data)?;
        let rgba_image = image.to_rgba8();
        
        let width = rgba_image.width() as usize;
        let height = rgba_image.height() as usize;
        
        let mut pixel_data = vec![vec![0u8; width]; height];
        
        // Convert each pixel to palette index
        for y in 0..height {
            for x in 0..width {
                let pixel = rgba_image.get_pixel(x as u32, y as u32);
                let Rgba([r, g, b, a]) = *pixel;
                
                // Handle transparency (alpha < 128 = transparent)
                // Use index 0 for transparency, but also check for pure black pixels with full alpha
                if a < 128 {
                    pixel_data[y][x] = 0; // Transparent = palette index 0
                    continue;
                }
                
                // Special case: pure black with full alpha should be index 1 (dark gray), not 0
                if r == 0 && g == 0 && b == 0 && a == 255 {
                    pixel_data[y][x] = 1; // Use dark gray instead of transparent black
                    continue;
                }
                
                // Find closest palette color
                let palette_index = self.find_closest_palette_color(r, g, b);
                pixel_data[y][x] = palette_index;
            }
        }
        
        // Generate Rust code
        let mut code = String::new();
        
        code.push_str(&format!("// {}x{} {} sprite\n", width, height, sprite_name));
        code.push_str(&format!("let {}_pixel_data = [\n", sprite_name.to_lowercase().replace(' ', "_")));
        
        for row in &pixel_data {
            code.push_str("    [");
            for (i, &pixel) in row.iter().enumerate() {
                if i > 0 { code.push(','); }
                code.push_str(&format!("{}", pixel));
            }
            code.push_str("],\n");
        }
        
        code.push_str("];\n\n");
        
        // Add color mapping information as comments
        code.push_str(&format!("// {} Color Usage:\n", sprite_name));
        let mut used_colors: Vec<u8> = pixel_data.iter()
            .flat_map(|row| row.iter().copied())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        used_colors.sort();
        
        for color_index in used_colors {
            let (r, g, b) = MASTER_PALETTE[color_index as usize];
            code.push_str(&format!("// {}: rgb({}, {}, {})\n", color_index, r, g, b));
        }
        
        Ok(code)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 3 {
        eprintln!("Usage: {} <input.png> <sprite_name>", args[0]);
        eprintln!("Example: {} hambert.png hambert", args[0]);
        std::process::exit(1);
    }
    
    let input_file = &args[1];
    let sprite_name = &args[2];
    
    // Read PNG file
    let png_data = fs::read(input_file)?;
    println!("Loaded PNG file: {} ({} bytes)", input_file, png_data.len());
    
    // Convert to sprite
    let mut converter = SpriteConverter::new();
    let rust_code = converter.convert_png_to_sprite(&png_data, sprite_name)?;
    
    // Output to file
    let output_file = format!("{}_sprite.rs", sprite_name);
    fs::write(&output_file, &rust_code)?;
    
    println!("Generated Rust code: {}", output_file);
    println!("\nFirst few lines of output:");
    println!("{}", rust_code.lines().take(10).collect::<Vec<_>>().join("\n"));
    
    Ok(())
}