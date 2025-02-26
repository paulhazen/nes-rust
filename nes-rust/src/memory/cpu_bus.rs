use crate::{cartridge::Cartridge, memory::bus::Bus};
use std::cell::Cell;

pub struct CPUBus {
    memory: Box<[u8]>,
    last_read_value: Cell<u8>,
    cycle_counter: Cell<u8>,
}

impl CPUBus {
    pub const RESET_VECTOR_DEFAULT: u16 = 0x8000;
}

impl Bus for CPUBus {

    fn load_cartridge(cartridge: Cartridge) -> Self {
        let mut memory = vec![0xFF; 0x10000].into_boxed_slice();

        // Properly set the reset vector
        memory[0xFFFC] = (Self::RESET_VECTOR_DEFAULT & 0xFF) as u8;
        memory[0xFFFD] = (Self::RESET_VECTOR_DEFAULT >> 8) as u8;

        let prg_rom_size = cartridge.prg_rom.len();
        let prg_rom_start = 0x8000;
    
        // Copy PRG-ROM into $8000-$BFFF
        memory[prg_rom_start..(prg_rom_start + prg_rom_size)]
            .copy_from_slice(&cartridge.prg_rom);
    
        // If PRG-ROM is only 16KB, mirror it into $C000-$FFFF
        if prg_rom_size == 0x4000 {
            memory[0xC000..=0xFFFF].copy_from_slice(&cartridge.prg_rom);
        }

        Self {
            memory,
            last_read_value: Cell::new(0xFF),
            cycle_counter: Cell::new(0x00),
        }
    }

    fn readable_ranges() -> &'static [std::ops::Range<u16>] {
        &[0x0000..0xFFFF]
    }

    fn writeable_ranges() -> &'static [std::ops::Range<u16>] {
        &[0x0000..0x1FF]
    }

    fn mirror_mask() -> &'static Option<u16> {
        &Some(0x07FF)
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

    fn memory(&self) -> &[u8] {
        &self.memory
    }

    fn memory_mut(&mut self) -> &mut [u8] {
        &mut self.memory
    }
}