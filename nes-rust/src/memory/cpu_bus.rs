use crate::{cartridge::Cartridge, memory::bus::Bus};
use std::cell::Cell;
<<<<<<< HEAD
use crate::util::print_hex_dump;
=======
>>>>>>> ac64b2fa59787330904af5315794eab496bbd747

pub struct CPUBus {
    memory: Box<[u8]>,
    last_read_value: Cell<u8>,
    cycle_counter: Cell<u8>,
}

impl CPUBus {
    pub const UNMAPPED: u8 = 0xFF;
    pub const RESET_VECTOR_ADDR: u16 = 0xFFFC;
    pub const RESET_VECTOR_HIGH_ADDR: u16 = 0xFFFD;
    pub const RESET_VECTOR_DEFAULT: u16 = 0x8000;
<<<<<<< HEAD
    pub const RAM_START: u16 = 0x0000;
    pub const RAM_END: u16 = 0x1FF;
=======
>>>>>>> ac64b2fa59787330904af5315794eab496bbd747
}

impl Bus for CPUBus {

    fn load_cartridge(cartridge: Cartridge) -> Self {
<<<<<<< HEAD
        let mut memory = vec![Self::UNMAPPED; 0x10000].into_boxed_slice();

        // Properly set the reset vector
        memory[Self::RESET_VECTOR_ADDR as usize] = (Self::RESET_VECTOR_DEFAULT & Self::UNMAPPED as u16) as u8;
        memory[Self::RESET_VECTOR_HIGH_ADDR as usize] = (Self::RESET_VECTOR_DEFAULT >> 8) as u8;

        let prg_rom_size = cartridge.prg_rom.len();
        let prg_rom_start = Self::RESET_VECTOR_DEFAULT as usize;
=======
        let mut memory = vec![0xFF; 0x10000].into_boxed_slice();

        // Properly set the reset vector
        memory[0xFFFC] = (Self::RESET_VECTOR_DEFAULT & 0xFF) as u8;
        memory[0xFFFD] = (Self::RESET_VECTOR_DEFAULT >> 8) as u8;

        let prg_rom_size = cartridge.prg_rom.len();
        let prg_rom_start = 0x8000;
>>>>>>> ac64b2fa59787330904af5315794eab496bbd747
    
        // Copy PRG-ROM into $8000-$BFFF
        memory[prg_rom_start..(prg_rom_start + prg_rom_size)]
            .copy_from_slice(&cartridge.prg_rom);
    
        // If PRG-ROM is only 16KB, mirror it into $C000-$FFFF
        if prg_rom_size == 0x4000 {
            memory[0xC000..=0xFFFF].copy_from_slice(&cartridge.prg_rom);
        }

        Self {
            memory,
<<<<<<< HEAD
            last_read_value: Cell::new(Self::UNMAPPED),
=======
            last_read_value: Cell::new(0xFF),
>>>>>>> ac64b2fa59787330904af5315794eab496bbd747
            cycle_counter: Cell::new(0x00),
        }
    }

    fn readable_ranges() -> &'static [std::ops::Range<u16>] {
<<<<<<< HEAD
        &[Self::RAM_START..0xFFFF]
    }

    fn writeable_ranges() -> &'static [std::ops::Range<u16>] {
        &[Self::RAM_START..Self::RAM_END]
=======
        &[0x0000..0xFFFF]
    }

    fn writeable_ranges() -> &'static [std::ops::Range<u16>] {
        &[0x0000..0x1FF]
>>>>>>> ac64b2fa59787330904af5315794eab496bbd747
    }

    fn mirror_mask() -> u16 {
        0x07FF
    }

    fn set_cycle_counter(&self, value: u8) {
        self.cycle_counter.set(value)
    }

    fn get_cycles(&self) -> u8 {
        self.cycle_counter.get()
    }

    fn set_last_read_value(&self, value: u8) {
        self.last_read_value.set(value)
    }

    fn get_last_read_value(&self) -> u8 {
        self.last_read_value.get()
    }

<<<<<<< HEAD
    fn memory(&self) -> &Box<[u8]> {
        &self.memory
    }

    fn memory_mut(&mut self) -> &mut Box<[u8]> {
=======
    fn memory(&self) -> &[u8] {
        &self.memory
    }

    fn memory_mut(&mut self) -> &mut [u8] {
>>>>>>> ac64b2fa59787330904af5315794eab496bbd747
        &mut self.memory
    }
}