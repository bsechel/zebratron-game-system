use zebratron_core::ZebratronCartridgeSystem;
use minifb::{Window, WindowOptions, Key};
use std::time::Duration;
use std::sync::{Arc, Mutex};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use gilrs::{Gilrs, Button, Event};

const WIDTH: usize = 320;
const HEIGHT: usize = 240;
const SAMPLE_RATE: u32 = 44100;

// Gamepad Thresholds (Kiwitata / Non-standard USB controller)
// These controllers often send a constant 0.44 signal for Right
const CONTROLLER_NEUTRAL_ZONE: f32 = 0.44;
const CONTROLLER_RIGHT_THRESHOLD: f32 = 0.70;
const CONTROLLER_LEFT_THRESHOLD: f32 = 0.20;

fn main() {
    let mut system = ZebratronCartridgeSystem::new();
    system.load_platformer_cartridge();
    system.start();

    // Wrap system in Arc<Mutex> for thread-safe audio access
    let system_arc = Arc::new(Mutex::new(system));

    // Audio setup - Make it non-fatal
    let host = cpal::default_host();
    let stream = match host.default_output_device() {
        Some(device) => {
            let config = device.default_output_config().unwrap();
            let system_audio = Arc::clone(&system_arc);
            
            let stream_result = device.build_output_stream(
                &config.into(),
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    if let Ok(mut sys) = system_audio.try_lock() {
                        for sample in data.iter_mut() {
                            *sample = sys.generate_audio_sample();
                        }
                    }
                },
                |err| eprintln!("🔊 Audio stream error: {}", err),
                None
            );

            match stream_result {
                Ok(s) => {
                    s.play().unwrap();
                    println!("🔊 Audio system initialized successfully");
                    Some(s)
                },
                Err(e) => {
                    eprintln!("🔊 Audio initialization failed: {}. Running in silent mode.", e);
                    None
                }
            }
        },
        None => {
            eprintln!("🔊 No audio output device found. Running in silent mode.");
            None
        }
    };

    // Keep the stream alive
    let _audio_stream = stream;

    // Gamepad setup
    let mut gilrs = Gilrs::new().unwrap();
    for (_id, gamepad) in gilrs.gamepads() {
        println!("🎮 Found gamepad: {}", gamepad.name());
    }

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
    println!("🔊 Audio Active: SID Synth & Sound Effects");
    println!("⌨️  Controls: Arrows = Move, Z = A Button, X = B Button");
    println!("🎮 Gamepad: D-Pad = Move, South = A Button, East = B Button");

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Drain gamepad events to keep internal state up to date
        while let Some(_) = gilrs.next_event() {}

        let mut sys = system_arc.lock().unwrap();

        // Keyboard input
        let mut up = window.is_key_down(Key::Up);
        let mut down = window.is_key_down(Key::Down);
        let mut left = window.is_key_down(Key::Left);
        let mut right = window.is_key_down(Key::Right);
        let mut a_button = window.is_key_down(Key::Z);
        let mut b_button = window.is_key_down(Key::X);

        // Combine with Gamepad input (first active gamepad)
        if let Some((_id, gamepad)) = gilrs.gamepads().next() {
            // Specialized Kiwitata / Non-standard controller mapping
            // Note: These controllers often map L/R to a single 'Right' button value
            if let Some(data) = gamepad.button_data(Button::DPadRight) {
                let val = data.value();
                if val > CONTROLLER_RIGHT_THRESHOLD { 
                    right = true; 
                } else if val < CONTROLLER_LEFT_THRESHOLD { 
                    left = true; 
                }
            }

            // Standard mappings for other directions and buttons
            if gamepad.is_pressed(Button::DPadUp) { up = true; }
            if gamepad.is_pressed(Button::DPadDown) { down = true; }
            if gamepad.is_pressed(Button::South) { a_button = true; } 
            if gamepad.is_pressed(Button::East) { b_button = true; }  
        }

        sys.handle_input(up, down, left, right, a_button, b_button);

        // Update frame
        if sys.step_frame() {
            // Actually draw the frame to the buffer
            sys.render();

            // Get RGBA buffer from system
            let rgba_buffer = sys.get_screen_buffer_vec();
            
            // Convert RGBA u8 to ARGB u32 for minifb
            for i in 0..(WIDTH * HEIGHT) {
                let r = rgba_buffer[i * 4] as u32;
                let g = rgba_buffer[i * 4 + 1] as u32;
                let b = rgba_buffer[i * 4 + 2] as u32;
                buffer[i] = (r << 16) | (g << 8) | b;
            }
        }

        // Release the lock before updating the window
        drop(sys);

        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}