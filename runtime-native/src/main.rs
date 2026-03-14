use zebratron_core::ZebratronCartridgeSystem;
use minifb::{Window, WindowOptions, Key};
use std::time::{Duration, Instant};

const WIDTH: usize = 320;
const HEIGHT: usize = 240;

fn main() {
    let mut system = ZebratronCartridgeSystem::new();
    system.load_platformer_cartridge();
    system.start();
    system.skip_to_gameplay();

    let mut window = Window::new(
        "Zebratron Game System - Native",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: minifb::Scale::X2,
            ..WindowOptions::default()
        },
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to 60 fps
    window.limit_update_rate(Some(Duration::from_micros(16600)));

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    println!("🎮 Zebratron Native Runtime Initialized");
    println!("⌨️  Controls: Arrows = Move, Z = A Button, X = B Button");

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Handle input
        let up = window.is_key_down(Key::Up);
        let down = window.is_key_down(Key::Down);
        let left = window.is_key_down(Key::Left);
        let right = window.is_key_down(Key::Right);
        let a_button = window.is_key_down(Key::Z);
        let b_button = window.is_key_down(Key::X);

        system.handle_input(up, down, left, right, a_button, b_button);

        // Update frame
        if system.step_frame() {
            // Actually draw the frame to the buffer
            system.render();

            // Get RGBA buffer from system
            let rgba_buffer = system.get_screen_buffer_vec();
            
            // Convert RGBA u8 to ARGB u32 for minifb
            for i in 0..(WIDTH * HEIGHT) {
                let r = rgba_buffer[i * 4] as u32;
                let g = rgba_buffer[i * 4 + 1] as u32;
                let b = rgba_buffer[i * 4 + 2] as u32;
                // a = rgba_buffer[i * 4 + 3] (ignored by XRGB)
                
                buffer[i] = (r << 16) | (g << 8) | b;
            }
        }

        // We unwrap here as we want the batch to fail if it can't update the window
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
