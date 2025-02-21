use super::ram::RAM;
use super::rom::ROM;
use crate::cartridge::{Cartridge};
use crate::memory::memory_view::MemoryView;

pub struct MemoryBus {
    memory: Box<[u8]>,
    pub ram: RAM,
    pub rom: ROM,
}

impl MemoryBus {
    pub const UNMAPPED: u8 = 0xFF;
    pub const RESET_VECTOR_ADDR: u16 = 0xFFFC;
    pub const RESET_VECTOR_HIGH_ADDR: u16 = 0xFFFD;
    pub const RESET_VECTOR_DEFAULT: u16 = 0x8000;

    pub fn load_cartridge(cartridge: Cartridge) -> Self {
        let mut memory = vec![0xFF; 0x10000].into_boxed_slice(); // Initialize all memory
    
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
        }
    }
    

    pub fn read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x1FFF => self.ram.read(&*self.memory, address),
            0x8000..=0xFFFF => self.memory[address as usize], // Direct PRG-ROM read
            _ => 0xFF,
        }
    }
    
    

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => self.ram.write(&mut self.memory, address, value),
            0x8000..=0xFFFF => {} // ROM is read-only
            _ => {},
        }
    }

    pub fn debug_prg_rom_mapping(&self, start: u16, end: u16) {
        println!("PRG-ROM Mapping from {:#06X} to {:#06X}:", start, end);
        for addr in (start..=end).step_by(16) { // Print every 16 bytes for readability
            let mapped_value = self.read(addr);
            println!("{:#06X}: {:#04X}", addr, mapped_value);
        }
    }

    // endregion: Helper Functions
}
