# Raspberry Pi Port Instructions

> **Disclaimer:** These are research notes. The first port attempt is currently in progress.

Guide for porting ZebratronGameSystem to Raspberry Pi hardware.

## Overview

Converting the WebAssembly-based ZebratronGameSystem to run natively on Raspberry Pi. The core game engine (written in Rust) can be largely reused - we just need to replace the web interface with native Linux graphics and input.

## Dual-Target Strategy (Web + Native)

**Good news**: The architecture makes it easy to support BOTH web and native Pi builds from the same codebase!

### Current Architecture Analysis

**Already portable (no changes needed):**
- ✅ Core game engine (CPU, PPU, APU, Memory)
- ✅ Cartridge system
- ✅ Tile/sprite systems
- ✅ Font rendering
- ✅ Audio synthesis (digital oscillators)

**Platform-specific (thin interface layer):**
- ❌ `#[wasm_bindgen]` attributes (web only)
- ❌ Canvas rendering (replaced with native framebuffer)
- ❌ Web Audio API callbacks (replaced with ALSA/CPAL)
- ❌ Browser input events (replaced with gamepad library)

### Simple Dual-Build Approach

**Don't overcomplicate it!** No need for complex trait abstractions or conditional compilation. Just separate the runtime layers:

```
ZebratronGameSystem/
├── core/                    # Shared engine (pure Rust, no wasm_bindgen)
│   ├── src/
│   │   ├── cpu.rs          # ✅ Works everywhere
│   │   ├── ppu.rs          # ✅ Works everywhere
│   │   ├── apu.rs          # ✅ Works everywhere
│   │   ├── memory.rs       # ✅ Works everywhere
│   │   ├── cartridge.rs    # ✅ Works everywhere
│   │   └── lib.rs          # Pure Rust exports (no wasm_bindgen)
│   └── Cargo.toml
│
├── runtime-web/             # Web runtime (current code)
│   ├── src/
│   │   ├── system.rs       # WASM bindings wrapper
│   │   ├── renderer.ts     # Canvas API
│   │   └── demo.ts         # Browser code
│   └── Cargo.toml          # Depends on core + wasm-bindgen
│
└── runtime-native/          # Native Pi runtime (new, simple)
    ├── src/
    │   └── main.rs         # minifb/SDL2 + CPAL + gilrs
    └── Cargo.toml          # Depends on core + native libraries
```

### How Both Runtimes Work

**Web runtime** (runtime-web/src/system.rs):
```rust
use wasm_bindgen::prelude::*;
use zebratron_core::{Cpu, Ppu, Apu, Memory};

#[wasm_bindgen]
pub struct ZebratronSystem {
    cpu: Cpu,      // From core
    ppu: Ppu,      // From core
    apu: Apu,      // From core
    memory: Memory // From core
}

#[wasm_bindgen]
impl ZebratronSystem {
    pub fn step_frame(&mut self) -> bool {
        // Same engine logic
    }
}
```

**Native runtime** (runtime-native/src/main.rs):
```rust
use zebratron_core::{Cpu, Ppu, Apu, Memory};
use minifb::{Window, WindowOptions};

struct NativeSystem {
    cpu: Cpu,      // Same core
    ppu: Ppu,      // Same core
    apu: Apu,      // Same core
    memory: Memory // Same core
}

fn main() {
    let mut system = NativeSystem::new();
    let mut window = Window::new("Zebratron", 320, 240, WindowOptions::default()).unwrap();

    while window.is_open() {
        system.step_frame(); // Same engine logic
        window.update_with_buffer(&system.get_framebuffer(), 320, 240).unwrap();
    }
}
```

### Why This Is Simple

1. **Core engine unchanged** - CPU/PPU/APU work identically everywhere
2. **Same game logic** - Cartridges use the same API on both platforms
3. **No code duplication** - Both runtimes are thin wrappers around core
4. **Easy maintenance** - Fix a bug once, both platforms benefit

### Build Commands

```bash
# Build for web (current workflow)
cd runtime-web
wasm-pack build --target web

# Build for Raspberry Pi native
cd runtime-native
cargo build --release --target aarch64-unknown-linux-gnu

# Or build directly on Pi
cargo build --release
```

### Migration Path

**Phase 1: Test kiosk mode first** (fastest)
- Use current web build in Chromium kiosk mode on Pi
- Validate hardware, benchmark performance
- **Time: 30 minutes setup**

**Phase 2: Native port** (if needed for performance)
- Restructure: Move core to separate crate
- Create runtime-native with minifb/CPAL/gilrs
- Keep runtime-web working
- **Time: 1-2 weeks**

**Phase 3: Optimize** (optional)
- GPU acceleration
- Custom Linux image
- Auto-boot configuration
- **Time: 2-3 weeks**

### The Bottom Line

**You're not rewriting anything.** You're just:
1. Moving core engine to library crate (refactor, not rewrite)
2. Adding a new native main.rs (200-300 lines)
3. Keeping web runtime working

The "complex trait abstraction" approach is overkill. This simpler structure is easier to understand and maintain.

## 2026 Dual-Target Evolution (Official Roadmap)

To maintain a fast web-based development cycle while ensuring a high-performance native Raspberry Pi experience, we are adopting a **Feature-Flagged Core** architecture.

### 1. Refactored Core (zebratron-core)
The `core/` crate will remain the "brain" of the system but will no longer be strictly tied to WASM.

**Cargo.toml Strategy:**
```toml
[dependencies]
# WASM dependencies become optional
wasm-bindgen = { version = "0.2", optional = true }
web-sys = { version = "0.3", optional = true }

[features]
default = ["wasm"]
wasm = ["wasm-bindgen", "web-sys"]
native = [] # For future native optimizations
```

### 2. The Native Runtime (runtime-native)
A new top-level crate will be created to host the native Pi/Desktop version.

- **Graphics:** `minifb` (Low-overhead framebuffer window).
- **Audio:** `cpal` (Native ALSA/PulseAudio output).
- **Input:** `gilrs` (Direct USB Gamepad/Controller support).

### 3. Unified Development Workflow
- **Web:** `wasm-pack build` (Enables `wasm` feature).
- **Pi:** `cargo run -p runtime-native` (Ignores `wasm` code).

---

## Physical Cartridge System (2026 Vision)

To move beyond "emulation" and create a true **Zebratron Game System** console, the Pi version supports a physical cartridge slot.

### 1. USB-Based Cartridges
The simplest implementation uses standard USB flash drives enclosed in a custom 3D-printed or CNC-machined wooden shell.

- **Hardware**: A standard USB-A port is routed to a physical "slot" on the console.
- **Software**: Linux `udev` rules detect the insertion of a drive and automatically mount it to `/mnt/cartridge`.
- **Logic**: The ZGS engine looks for a `cartridge.zgs` file with a "Magic Number" (e.g., `0x5A454252` - "ZEBR") to verify authenticity before booting.

### 2. Wider Format Card Edge Connectors
For a more authentic retro feel, the ZGS uses a "Wider Format" card edge connector (e.g., a 36-pin or 44-pin industrial socket).

- **The Slot**: A standard industrial card edge connector is mounted inside the wooden enclosure.
- **The Cartridge**: A custom PCB with gold fingers is enclosed in a wide wooden shell. 
- **Pin Assignment**:
    - **Pins 1-4**: USB Data (for the game engine).
    - **Pins 5-6**: Direct MIDI (UART) for low-latency synth input.
    - **Pins 7-8**: High-quality Analog Audio Out (bypassing Pi PWM).
    - **Expansion**: Future pins for additional knobs/sliders on the cartridge itself.

### 3. "Fail-Safe" Cartridge Logic
Following the project's "fail-safe" mandates:
- **Hot-Swapping**: Pulling a cartridge during play triggers an immediate "Freeze & Return to Splash" state.
- **Auto-Boot**: The console powers on to a splash screen and only starts the engine once a valid "ZEBR" signature is detected.
- **Persistence**: Game saves and synth presets are written directly back to the cartridge, making it truly portable between ZGS consoles.

---

## Hardware Integration (2026)

### USB Game Controller Support
The native runtime will use the `gilrs` crate to provide raw, low-latency access to wired USB controllers. This bypasses the browser's Gamepad API and provides the "zero-lag" feel required for the Miracle Mountaintop platforming.


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
let stream = device.build_output_stream(
    &config.into(),
    move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
        for sample in data.iter_mut() {
            *sample = system.generate_audio_sample(); // Same APU method!
        }
    },
    |err| eprintln!("Audio error: {}", err)
).unwrap();

stream.play().unwrap();
```

## Detailed Audio Architecture

### Audio Processing Flow Comparison

**Web Version (Current):**
```
Rust APU → WASM → JavaScript → Web Audio API → Browser Audio → Speakers
```

**Raspberry Pi Version:**
```
Rust APU → ALSA/PulseAudio/JACK → Linux Audio → Audio Hardware → Speakers
```

### Key Insight: Core APU Stays Identical

The **generate_audio_sample()** method in our APU works identically across platforms:

```rust
// This exact same code works on web AND Raspberry Pi!
impl ZebratronSystem {
    pub fn generate_audio_sample(&mut self) -> f32 {
        self.apu.generate_sample() // Same digital oscillators everywhere!
    }
}
```

### Pi Audio Output Options

#### Option 1: ALSA (Direct Hardware - Recommended)

```rust
// Cargo.toml additions
[dependencies]
alsa = "0.7"

use alsa::{Direction, ValueOr};
use alsa::pcm::{PCM, HwParams, Format, Access};

pub struct PiAudioOutput {
    pcm: PCM,
    buffer: Vec<f32>,
}

impl PiAudioOutput {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let pcm = PCM::new("default", Direction::Playback, false)?;

        // Configure for retro gaming
        {
            let hwp = HwParams::any(&pcm)?;
            hwp.set_channels(1)?; // Mono for authentic retro feel
            hwp.set_rate(44100, ValueOr::Nearest)?;
            hwp.set_format(Format::s16())?;
            hwp.set_access(Access::RWInterleaved)?;
            hwp.set_buffer_size(1024)?; // Low latency buffer
            pcm.hw_params(&hwp)?;
        }

        Ok(PiAudioOutput {
            pcm,
            buffer: vec![0.0; 1024]
        })
    }

    pub fn audio_callback(&mut self, system: &mut ZebratronSystem) -> Result<(), Box<dyn std::error::Error>> {
        // Fill buffer with samples from our APU (same as web!)
        for i in 0..self.buffer.len() {
            self.buffer[i] = system.generate_audio_sample();
        }

        // Convert f32 to i16 and send to hardware
        let i16_samples: Vec<i16> = self.buffer.iter()
            .map(|&sample| (sample.clamp(-1.0, 1.0) * 32767.0) as i16)
            .collect();

        self.pcm.io_i16()?.writei(&i16_samples)?;
        Ok(())
    }
}
```

**Latency: ~5-15ms** (Much better than web's ~20-50ms!)

#### Option 2: JACK Audio (Professional/Studio)

```rust
// For ultra-low latency professional audio
[dependencies]
jack = "0.11"

use jack::prelude::*;

pub struct JackAudioOutput {
    _client: jack::AsyncClient<(), ProcessHandler>,
}

struct ProcessHandler {
    system: ZebratronSystem,
    audio_out: jack::Port<jack::AudioOut>,
}

impl jack::ProcessHandler for ProcessHandler {
    fn process(&mut self, _: &jack::Client, ps: &jack::ProcessScope) -> jack::Control {
        let audio_buffer = self.audio_out.as_mut_slice(ps);

        // Fill JACK buffer with our APU (same oscillators!)
        for sample in audio_buffer.iter_mut() {
            *sample = self.system.generate_audio_sample();
        }

        jack::Control::Continue
    }
}
```

**Latency: ~1-5ms** (Professional grade - perfect for live performance!)

### Pi Hardware Audio Options

#### Built-in Audio Outputs:
1. **3.5mm Headphone Jack**
   - All Pi models have this
   - Basic quality (fine for retro gaming)
   - Zero additional cost

2. **HDMI Audio**
   - Digital output through HDMI
   - Good quality when connected to TV/monitor
   - Automatic device switching

#### Professional Audio HATs:
3. **HiFiBerry DAC+ (~$35)**
   ```bash
   # Enable in /boot/config.txt
   dtoverlay=hifiberry-dac

   # High-quality I2S DAC
   # Perfect for synthesizer applications
   ```

4. **IQaudio Pi-DAC+ (~$30)**
   ```bash
   dtoverlay=iqaudio-dac

   # Low-noise design
   # Great for clean audio output
   ```

5. **USB Audio Interface**
   - Professional studio equipment compatibility
   - Multiple inputs/outputs for complex setups
   - Examples: Focusrite Scarlett, Behringer UCA202

### Main Audio Loop Integration

```rust
// main.rs - Complete Pi audio system
use zebratron_core::ZebratronSystem;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut system = ZebratronSystem::new();
    let mut audio = PiAudioOutput::new()?;

    println!("🎵 ZebratronGameSystem Pi Edition");
    println!("🎛️  Digital Oscillators: Ready");
    println!("🔊 Audio Hardware: Initialized");

    // Enter sound test mode for demo
    system.enter_sound_test_mode();

    loop {
        // Generate audio samples (same APU as web!)
        audio.audio_callback(&mut system)?;

        // Handle other system updates
        system.step_frame();

        // Maintain 60fps timing
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}
```

### Audio Configuration Examples

#### Set Pi Audio Device:
```bash
# List available audio devices
aplay -l

# Set default output device
sudo raspi-config
# Advanced Options → Audio → Force 3.5mm ('headphones')
```

#### Enable Audio HAT:
```bash
# For HiFiBerry DAC+
echo "dtoverlay=hifiberry-dac" | sudo tee -a /boot/config.txt
sudo reboot

# Verify it's working
speaker-test -c1 -t wav
```

#### Low-Latency Setup:
```bash
# Install real-time kernel (optional)
sudo apt install linux-image-rt-armv7

# Optimize audio settings
echo "@audio - rtprio 95" | sudo tee -a /etc/security/limits.conf
echo "@audio - memlock unlimited" | sudo tee -a /etc/security/limits.conf
```

### Performance Comparison

| Platform | Typical Latency | Best Latency | Notes |
|----------|----------------|--------------|-------|
| Web Browser | 20-50ms | 10ms | Varies by browser |
| Pi + ALSA | 5-15ms | 3ms | Direct hardware |
| Pi + JACK | 1-5ms | <1ms | Professional grade |
| Pi + PulseAudio | 15-30ms | 8ms | User-friendly |

**The Pi version will have BETTER audio latency than web!** 🎯

### Synthesizer-Specific Optimizations

#### For Retro Synth Musicians:
```rust
// Optimize buffer sizes for real-time control
const AUDIO_BUFFER_SIZE: usize = 128; // ~3ms latency

// Add MIDI input support
[dependencies]
midir = "0.9"

// Real-time parameter updates
impl ZebratronSystem {
    pub fn midi_note_on(&mut self, note: u8, velocity: u8) {
        self.sound_test_change_note(note);
        // Velocity could control volume/filter cutoff
    }

    pub fn midi_cc(&mut self, controller: u8, value: u8) {
        match controller {
            1 => self.sound_test_set_pulse_width(value as f32 / 127.0),
            74 => self.sound_test_set_detune((value as f32 - 64.0) / 127.0),
            // Add more CC mappings
            _ => {}
        }
    }
}
```

**The Pi version will be perfect for live synthesizer performance with sub-5ms latency!** 🎹

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

---

## Beyond the Pi: Custom Hardware Kits (Future Roadmap)

While the Raspberry Pi is an excellent development and "Mini-PC" platform, the ultimate goal for the ZebratronGameSystem is a **dedicated hardware console kit**. This involves moving away from a full OS (Linux) to "bare metal" microcontrollers.

### 1. The "Brain" (Microcontroller Selection)
Since the Zebratron engine is pure Rust, it can be compiled for `#![no_std]` environments (no operating system).

- **RP2040 (Raspberry Pi Pico):** 
    - **Pros:** Excellent Rust support, dual-core, unique PIO (Programmable I/O) for custom video/audio signals.
    - **Cost:** ~$1.00 per chip.
- **ESP32-S3:**
    - **Pros:** Built-in WiFi/Bluetooth, faster clock speed (240MHz), great for "Connected Console" features.
    - **Cost:** ~$3.00 per chip.

### 2. Custom PCB Design
A custom Zebratron PCB would integrate the microcontroller, display connector, battery charging, and physical controls.

- **Prototyping:** 5-10 boards for ~$50-100 via services like JLCPCB.
- **Production:** Fully assembled boards for ~$5-10 per unit at scale.

### 3. Estimated Hardware Kit BOM (100+ units)

| Component | Estimated Cost (per unit) |
| :--- | :--- |
| Assembled Custom PCB (RP2040/ESP32) | $7.00 - $12.00 |
| 3.5" - 5" SPI/Parallel LCD Screen | $10.00 - $18.00 |
| Silicone Buttons & D-Pad Membranes | $2.00 - $4.00 |
| LiPo Battery & Power Management | $5.00 - $8.00 |
| Enclosure (3D Printed or Injection Molded) | $4.00 - $10.00 |
| **Total Hardware Target** | **$28.00 - $52.00** |

### 4. Technical Migration Path
1. **Refactor for `#![no_std]`:** Ensure `zebratron-core` doesn't rely on the standard library (already partially done via the WASM decoupling).
2. **Hardware Drivers:** Write simple Rust drivers for the specific LCD controller (e.g., ST7789) and audio DAC.
3. **Firmware Host:** Create a tiny `main.rs` that loops:
    - Read physical pins (Buttons).
    - `system.step_frame()`.
    - Push `screen_buffer` to LCD via high-speed SPI/Parallel.

### 5. Value Proposition
A dedicated Zebratron kit isn't just a generic emulator; it is a **bespoke creative tool**. By combining the game system with a native **Moroder Phaser** synth and high-quality hardware controls, it becomes a "Pocket Studio" for 8-bit musicians and gamers alike.