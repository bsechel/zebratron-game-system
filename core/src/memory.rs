use wasm_bindgen::prelude::*;

const MAIN_RAM_SIZE: usize = 64 * 1024; // 64KB main RAM
const VIDEO_RAM_SIZE: usize = 32 * 1024; // 32KB video RAM

#[wasm_bindgen]
pub struct Memory {
    main_ram: Vec<u8>,
    video_ram: Vec<u8>,
    cartridge_rom: Vec<u8>,
}

#[wasm_bindgen]
impl Memory {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Memory {
        Memory {
            main_ram: vec![0; MAIN_RAM_SIZE],
            video_ram: vec![0; VIDEO_RAM_SIZE],
            cartridge_rom: Vec::new(),
        }
    }

    pub fn load_cartridge(&mut self, rom_data: &[u8]) {
        self.cartridge_rom = rom_data.to_vec();
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            // Main RAM: 0x0000 - 0xFFFF
            0x0000..=0x7FFF => {
                if (address as usize) < self.main_ram.len() {
                    self.main_ram[address as usize]
                } else {
                    0
                }
            }
            // Cartridge ROM: 0x8000 - 0xFFFF
            0x8000..=0xFFFF => {
                let rom_address = (address - 0x8000) as usize;
                if rom_address < self.cartridge_rom.len() {
                    self.cartridge_rom[rom_address]
                } else {
                    0
                }
            }
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            // Main RAM: 0x0000 - 0x7FFF
            0x0000..=0x7FFF => {
                if (address as usize) < self.main_ram.len() {
                    self.main_ram[address as usize] = value;
                }
            }
            // ROM area is read-only, ignore writes
            0x8000..=0xFFFF => {
                // Ignore writes to ROM
            }
        }
    }

    pub fn read_video_byte(&self, address: u16) -> u8 {
        if (address as usize) < self.video_ram.len() {
            self.video_ram[address as usize]
        } else {
            0
        }
    }

    pub fn write_video_byte(&mut self, address: u16, value: u8) {
        if (address as usize) < self.video_ram.len() {
            self.video_ram[address as usize] = value;
        }
    }
}