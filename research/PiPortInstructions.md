# Raspberry Pi Port Instructions

Guide for porting ZebratronGameSystem to Raspberry Pi hardware.

## Overview

Converting the WebAssembly-based ZebratronGameSystem to run natively on Raspberry Pi. The core game engine (written in Rust) can be largely reused - we just need to replace the web interface with native Linux graphics and input.

## Implementation Options

### Option A: Native Rust with minifb (Easiest - 1-2 weeks)
- **Pros**: Minimal changes, quick development, maintains existing architecture
- **Cons**: Requires Linux desktop environment
- **Best for**: Development kits, proof of concept

### Option B: Embedded Linux (2-3 weeks)
- **Pros**: Console-like experience, boots directly to games
- **Cons**: More complex setup, custom Linux image required
- **Best for**: Commercial product, authentic console feel

### Option C: Bare Metal (4+ weeks)
- **Pros**: Ultimate performance, no OS overhead, authentic retro feel
- **Cons**: Much more complex, lose debugging tools
- **Best for**: Hardware enthusiasts, maximum authenticity

## Hardware Requirements

### Recommended: Raspberry Pi 4
- **CPU**: Quad-core Cortex-A72 @ 1.8GHz
- **RAM**: 2GB minimum (4GB recommended)
- **GPU**: VideoCore VI (hardware acceleration)
- **Storage**: 32GB+ SD card
- **Cost**: ~$35-55

### Alternatives:
- **Pi 400**: Built-in keyboard version
- **Pi Zero 2W**: Lower cost option (~$15)
- **CM4**: For custom hardware integration

## Implementation Steps

### Week 1: Core Engine Port

1. **Remove WebAssembly bindings**:
```rust
// Remove these from lib.rs
// use wasm_bindgen::prelude::*;
// #[wasm_bindgen]

// Replace with standard Rust exports
pub use system::ZebratronSystem;
pub use ppu::Ppu;
// etc.
```

2. **Add native dependencies** (Cargo.toml):
```toml
[dependencies]
minifb = "0.25"          # Window/framebuffer
gilrs = "0.10"           # Controller input
cpal = "0.15"            # Audio output
serde_json = "1.0"       # Config files
```

3. **Create native main loop**:
```rust
use minifb::{Key, Window, WindowOptions};

fn main() {
    let mut system = ZebratronSystem::new();
    let mut window = Window::new(
        "ZebratronGameSystem",
        320, 240,
        WindowOptions::default()
    ).unwrap();

    // Main game loop
    while window.is_open() {
        // Handle input
        // Step system
        // Render frame
        window.update_with_buffer(&screen_buffer, 320, 240).unwrap();
    }
}
```

### Week 2: Input & Audio

1. **Gamepad support**:
```rust
use gilrs::{Gilrs, Button, Event, EventType};

let mut gilrs = Gilrs::new().unwrap();

// In game loop
while let Some(Event { id, event, time }) = gilrs.next_event() {
    match event {
        EventType::ButtonPressed(Button::DPadUp, _) => {
            // Handle up button
        }
        // Handle other buttons
        _ => {}
    }
}
```

2. **Audio output**:
```rust
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

// Set up audio stream
let host = cpal::default_host();
let device = host.default_output_device().unwrap();
let config = device.default_output_config().unwrap();

// Create audio callback that pulls samples from APU
```

### Week 3: Pi-Specific Optimizations

1. **GPU acceleration**:
```rust
// Use Pi's GPU for fast blitting
// Consider SDL2 instead of minifb for hardware acceleration
```

2. **Custom Linux image**:
```bash
# Use Buildroot or Yocto to create minimal Linux
# Include only necessary components:
# - Graphics drivers
# - Audio drivers
# - Your game executable
# - Controller drivers
```

3. **Boot configuration**:
```bash
# /boot/config.txt optimizations
gpu_mem=128              # More GPU memory
hdmi_group=2             # Custom resolution
hdmi_mode=87
hdmi_cvt=320 240 60      # Exact system resolution
```

### Week 4: Polish & Distribution

1. **Auto-boot setup**:
```bash
# /etc/systemd/system/zebratron.service
[Unit]
Description=ZebratronGameSystem
After=multi-user.target

[Service]
Type=simple
ExecStart=/usr/local/bin/zebratron-system
Restart=always
User=pi

[Install]
WantedBy=multi-user.target
```

2. **SD card "cartridges"**:
```bash
# Games stored as ROM files
# Hot-swappable SD cards act like cartridges
# File structure:
/boot/games/
  ├── mario-clone.zgs
  ├── space-shooter.zgs
  └── puzzle-game.zgs
```

## Development Kit Specifications

### Hardware BOM (~$100 total)
- Raspberry Pi 4 (4GB): $55
- Case/cooling: $15
- MicroSD card (64GB): $12
- USB controllers (2x): $20
- HDMI cable: $8
- Power supply: $10

### Software Stack
- **OS**: Custom Linux (Buildroot-based)
- **Runtime**: Native Rust executable
- **Graphics**: Direct framebuffer or SDL2
- **Audio**: ALSA direct output
- **Input**: Raw evdev or SDL2
- **Boot time**: < 10 seconds to games

## Performance Expectations

### Raspberry Pi 4 Capabilities
- **320x240 @ 60fps**: Easily achievable
- **640x480 @ 60fps**: Possible with optimization
- **Multiple sprites**: 50+ without issues
- **Audio**: 8+ channels, effects possible
- **RAM usage**: < 100MB total system

### Optimizations
- **GPU blitting**: Hardware-accelerated sprite compositing
- **CPU affinity**: Pin game loop to dedicated core
- **Memory layout**: Optimize cache usage for 60fps
- **Overclocking**: Pi 4 can run 2.0GHz+ safely

## Distribution Strategy

### Development Kit Model
1. **Custom PCB**: Pi Compute Module 4 integration
2. **Retro case**: NES/Genesis inspired design
3. **Built-in controllers**: Detachable or wired
4. **Cartridge slot**: SD card mechanism
5. **Price target**: $149-199

### Software Distribution
- **Web store**: Download ROMs for Pi
- **Development tools**: Cross-compilation toolchain
- **Documentation**: Porting guide for web games

## Next Steps

1. **Proof of concept**: Get basic rendering working on Pi
2. **Performance testing**: Measure fps, optimize bottlenecks
3. **Controller mapping**: Support popular USB gamepads
4. **Audio pipeline**: Implement APU audio output
5. **Game porting**: Convert existing web games
6. **Hardware design**: Custom case, PCB layout

## Risk Factors

### Technical
- **Performance**: Ensure 60fps with multiple sprites
- **Audio latency**: Real-time audio challenging on Linux
- **Controller compatibility**: Support wide range of gamepads

### Business
- **Pi availability**: Supply chain issues affect Pi 4
- **Competition**: Existing retro handhelds
- **Development complexity**: Native development harder than web

## Success Metrics

- **Boot time**: < 10 seconds from power-on to game
- **Performance**: Consistent 60fps gaming
- **Compatibility**: 90%+ of web games work on Pi
- **Latency**: < 50ms input lag
- **Battery life**: 4+ hours portable (with battery pack)

## Conclusion

Porting to Raspberry Pi is very feasible given the existing Rust codebase. The web version proves the concept works - Pi version just changes the frontend while keeping the proven game engine core.

Most challenging aspects will be audio pipeline and performance optimization, but Pi 4 has more than enough power for the ZebratronGameSystem specifications.