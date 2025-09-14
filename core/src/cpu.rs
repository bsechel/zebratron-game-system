use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Cpu {
    // 8-bit registers
    pub a: u8,    // Accumulator
    pub x: u8,    // X index register
    pub y: u8,    // Y index register
    pub sp: u8,   // Stack pointer
    pub pc: u16,  // Program counter
    pub status: u8, // Status flags

    // Cycle counter
    pub cycles: u64,
}

#[wasm_bindgen]
impl Cpu {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Cpu {
        Cpu {
            a: 0,
            x: 0,
            y: 0,
            sp: 0xFD,  // Stack pointer starts at 0x01FD
            pc: 0x8000, // Program counter starts at reset vector
            status: 0x24, // IRQ disabled, unused bit set
            cycles: 0,
        }
    }

    pub fn reset(&mut self) {
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.sp = 0xFD;
        self.pc = 0x8000;
        self.status = 0x24;
        self.cycles = 0;
    }

    pub fn step(&mut self) -> u8 {
        // Placeholder for instruction execution
        // This will be implemented with proper instruction decoding
        self.cycles += 1;
        1 // Return cycles taken for this instruction
    }

    // Status flag helpers
    pub fn get_carry_flag(&self) -> bool {
        (self.status & 0x01) != 0
    }

    pub fn set_carry_flag(&mut self, value: bool) {
        if value {
            self.status |= 0x01;
        } else {
            self.status &= !0x01;
        }
    }

    pub fn get_zero_flag(&self) -> bool {
        (self.status & 0x02) != 0
    }

    pub fn set_zero_flag(&mut self, value: bool) {
        if value {
            self.status |= 0x02;
        } else {
            self.status &= !0x02;
        }
    }
}