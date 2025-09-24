mod utils;
mod cpu;
mod ppu;
mod ppu_clean;
mod apu;
mod memory;
mod system;
mod system_cartridge;
mod cartridge;
mod sprite_converter;
mod laugh_sample;
mod font_system;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, ZebratronGameSystem!");
}

// Export the main system struct and cartridge
pub use system::ZebratronSystem;
pub use system_cartridge::ZebratronCartridgeSystem;
pub use cartridge::HambertCartridge;
pub use cartridge::ZSynthCartridge;
pub use sprite_converter::{SpriteConverter, SpriteData};