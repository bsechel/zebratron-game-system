# ZebratronGameSystem Developer Guide

## System Overview

ZebratronGameSystem is a custom 8-bit style game system inspired by classic retro gaming consoles. It runs in web browsers using WebAssembly for the core system and JavaScript/TypeScript for the runtime layer.

## Architecture

### Core Components (Rust/WebAssembly)

The core system is written in Rust and compiled to WebAssembly for performance-critical operations:

#### `/core/src/lib.rs`
- Main WebAssembly module entry point
- Exports public API for JavaScript integration
- Manages module initialization and memory allocation

#### `/core/src/system.rs` & `/core/src/system_cartridge.rs`
- **ZebratronSystem**: Basic system implementation
- **ZebratronCartridgeSystem**: Enhanced system with cartridge support
- Manages overall system state, timing, and component coordination
- Handles game loop execution and frame timing

#### `/core/src/cpu.rs`
- 8-bit CPU emulation
- Instruction set implementation
- Memory access and program counter management
- Designed to simulate classic 8-bit processor behavior

#### `/core/src/ppu.rs` & `/core/src/ppu_clean.rs`
- **PPU (Picture Processing Unit)**: Graphics rendering engine
- **ppu_clean.rs**: Optimized rendering implementation
- Sprite rendering with 8-bit style graphics
- Palette management and color mapping
- Background and sprite layer composition
- Visual effects (flashing, transparency)

#### `/core/src/apu.rs`
- **APU (Audio Processing Unit)**: Sound synthesis engine
- Chip-tune style audio generation
- Multiple audio channels for different sound types
- Sound effect management and audio state control
- Oscillator-based sound synthesis

#### `/core/src/memory.rs`
- Memory management and address space mapping
- Cartridge memory interface
- System RAM and video memory management

#### `/core/src/cartridge.rs`
- Game cartridge system implementation
- **HambertCartridge**: Example platformer game
- **ZSynthCartridge**: Audio synthesis cartridge
- Entity system for game objects
- Physics simulation (gravity, collision detection, platforms)
- Game state management (intro, playing, game over)

#### `/core/src/sprite_converter.rs`
- Sprite data conversion utilities
- PNG to sprite data processing
- Color palette optimization

### Runtime Layer (TypeScript/JavaScript)

The runtime provides browser integration and high-level APIs:

#### `/runtime/src/system.ts`
- Main system wrapper classes
- WebAssembly module loading and initialization
- Bridge between JavaScript and Rust core
- System lifecycle management

#### `/runtime/src/renderer.ts`
- Canvas-based rendering interface
- WebGL context management
- Frame buffer handling and display
- Screen scaling and viewport management

#### `/runtime/src/input.ts`
- Keyboard and gamepad input handling
- Input mapping and state management
- Button press/release event processing
- Multiple input device support

#### `/runtime/src/audio.ts`
- Web Audio API integration
- Audio buffer management
- Sound effect playback coordination
- Audio context lifecycle management

#### `/runtime/src/midi.ts`
- MIDI device integration
- Musical input for sound synthesis
- Real-time MIDI event processing

### Tools and Asset Pipeline

#### `/tools/`
- **png_to_sprite.rs**: PNG to sprite data converter
- **asset-converter/**: Asset processing utilities
- Sprite data files for game assets
- Build tools and automation scripts

## Game Development Model

### Cartridge System
Games are implemented as "cartridges" - self-contained modules that plug into the system:

1. **Cartridge Structure**: Each cartridge implements core game logic, entity management, and state
2. **Entity System**: Game objects are managed as entities with position, velocity, sprites, and behavior
3. **Physics Integration**: Built-in gravity, collision detection, and platform mechanics
4. **Audio Integration**: Sound effects and music through the APU system

### Development Workflow

1. **Core Development** (Rust):
   - Implement game logic in cartridge modules
   - Define sprites, entities, and game mechanics
   - Build with `wasm-pack` to generate WebAssembly

2. **Runtime Integration** (TypeScript):
   - Load and initialize the WebAssembly module
   - Set up rendering, input, and audio systems
   - Handle browser-specific concerns

3. **Asset Pipeline**:
   - Convert PNG graphics to sprite data
   - Generate color palettes for 8-bit style
   - Process audio assets for chip-tune synthesis

## Execution Model and Physics

### Frame-Based Execution Flow

The ZebratronGameSystem uses a **frame-based execution model** rather than a traditional game loop:

1. **Frame Stepping** (`system_cartridge.rs:82-121`):
   - `step_frame()` is called once per frame from the browser's animation loop
   - PPU (graphics) steps until a complete frame is rendered
   - APU (audio) processes sound effects
   - Cartridge game logic updates once per frame
   - Data is synchronized between cartridge and rendering systems

2. **Input Handling** (`system_cartridge.rs:124-144`):
   - Input events trigger `handle_input()` separately from frame stepping
   - Input is passed to the active cartridge's `update_game()` method
   - Immediate synchronization with PPU and audio systems

3. **Data Flow**:
   ```
   Browser Animation Frame → step_frame() → Cartridge Update → Sync to PPU/APU
   Browser Input Events → handle_input() → Cartridge Input → Sync to PPU/APU
   ```

### Physics and Movement System

The physics system in `cartridge.rs` follows a **velocity-based approach**:

#### Movement Physics (`cartridge.rs:263-372`):
- **Gravity**: Constant downward acceleration (0.15 units/frame)
- **Terminal Velocity**: Maximum fall speed (3.5 units/frame) 
- **Entity Velocity**: Each entity has `vel_x` (horizontal) and `vel_y` (vertical) components
- **Position Updates**: `entity.x += entity.vel_x` and `entity.y += entity.vel_y` each frame

#### Player Controls (`cartridge.rs:239-261`):
- **Horizontal Movement**: Left/right input sets velocity (-2.5 to +2.5)
- **Jumping**: Up input when on ground sets upward velocity (-6.5)
- **Velocity Damping**: Horizontal velocity is gradually reduced for realistic movement

#### Collision Detection System:

**Ground Collision** (`cartridge.rs:362-371`):
- Checks if entity bottom edge (`y + height`) reaches ground level (200.0)
- Sets `vel_y = 0.0` and `on_ground = true`
- Prevents entities from falling through the world floor

**Platform Collision** (`cartridge.rs:373-406`):
- Iterates through all platform entities
- Checks bounding box overlap with downward-moving entities
- Only triggers when falling (`vel_y >= 0.0`) to allow jumping through platforms
- Snaps entity to platform top and stops downward movement

**Enemy Collision** (`cartridge.rs:408-530`):
- Player vs Enemy: Damage system with health reduction
- Player vs Projectile: Hit detection with sound effects
- Ninja AI: Automatic shuriken throwing at player when in range

#### Special Physics Cases:
- **Death Animation**: Bypasses normal collision detection, allows falling through everything
- **Projectiles**: Custom gravity (0.1) and trajectory physics
- **Camera Following**: Smooth camera tracking of player movement

### Physics Design Principles:
- **Frame-Rate Independent**: All movement values are per-frame constants
- **Immediate Response**: Input handling is separate from frame updates for responsiveness
- **Entity-Component Model**: All game objects share the same physics and collision systems
- **Layered Collision**: Ground, platforms, and entity interactions are handled separately
- **State-Driven**: Physics behavior changes based on entity state (dying, on_ground, etc.)

This creates a responsive, arcade-style physics system typical of classic 2D platformers, with predictable movement and collision behavior.

## Key Design Principles

- **8-bit Aesthetic**: Limited color palettes, pixel-perfect graphics, chip-tune audio
- **Performance**: WebAssembly for CPU-intensive operations, optimized rendering
- **Modularity**: Cartridge system allows independent game development
- **Browser Compatibility**: Works in modern web browsers without plugins
- **Developer Friendly**: TypeScript APIs with good tooling support

## Getting Started

See [CLAUDE.md](./CLAUDE.md) for build instructions and development setup.

For examples of cartridge development, see:
- `HambertCartridge` in `/core/src/cartridge.rs` - Platformer game example
- `ZSynthCartridge` in `/core/src/cartridge.rs` - Audio synthesis example