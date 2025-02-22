use super::ram::RAM;
use super::rom::ROM;
use crate::cartridge::{Cartridge};
use crate::memory::memory_view::MemoryView;
use std::cell::Cell;

pub struct MemoryBus {
    memory: Box<[u8]>,
    ram: RAM,
    rom: ROM,
    last_read_value: Cell<u8>, // Cell allows the value to be changed without &mut self.
}

impl MemoryBus {
    pub const UNMAPPED: u8 = 0xFF;
    pub const RESET_VECTOR_ADDR: u16 = 0xFFFC;
    pub const RESET_VECTOR_HIGH_ADDR: u16 = 0xFFFD;
    pub const RESET_VECTOR_DEFAULT: u16 = 0x8000;

    pub fn load_cartridge(cartridge: Cartridge) -> Self {
        let mut memory = vec![0xFF; 0x10000].into_boxed_slice(); // Initialize all memory
    
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
            ram: RAM::new(0x0000, 0x07FF),
            rom: ROM::new(0x8000, 0xFFFF),
            last_read_value: Cell::new(0xFF), // For debugging, it may be wise to randomize this value
        }
    }

    pub fn read_word(&self, address: u16) -> u16 {
        let low_byte = self.read_byte(address) as u16;
        let high_byte = self.read_byte(address.wrapping_add(1)) as u16;
        (high_byte << 8) | low_byte
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        let value = match address {
            0x0000..=0x1FFF => self.ram.read(&*self.memory, address),
            0xFFFC => self.memory[0xFFFC], // Explicitly handle the Reset Vector
            0xFFFD => self.memory[0xFFFD],
            0x8000..=0xFFFF => self.memory[address as usize], // Direct PRG-ROM read
            _ => {
                self.last_read_value.get()
            }
        };

        self.last_read_value.set(value);
        value
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => self.ram.write(&mut self.memory, address, value),
            0x8000..=0xFFFF => {} // ROM is read-only
            _ => {},
        }
    }

    pub fn debug_view_reset_vector(&mut self) {
        println!(
            "Reset Vector: {:#06x} -> {:#06x} (Stored: {:#04x} {:#04x}, PC={:#06x})",
            MemoryBus::RESET_VECTOR_ADDR,
            MemoryBus::RESET_VECTOR_HIGH_ADDR,
            self.read_byte(MemoryBus::RESET_VECTOR_ADDR),
            self.read_byte(MemoryBus::RESET_VECTOR_HIGH_ADDR),
            ((self.read_byte(MemoryBus::RESET_VECTOR_HIGH_ADDR) as u16) << 8) | (self.read_byte(MemoryBus::RESET_VECTOR_ADDR) as u16)
        );        
    }

    pub fn debug_prg_rom_mapping(&self, start: u16, end: u16) {
        println!("PRG-ROM Mapping from {:#06X} to {:#06X}:", start, end);
        for addr in (start..=end).step_by(16) { // Print every 16 bytes for readability
            let mapped_value = self.read_byte(addr);
            println!("{:#06X}: {:#04X}", addr, mapped_value);
        }
    }

    // endregion: Helper Functions
}
