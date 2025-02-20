use super::ram::RAM;
use super::rom::ROM;
use crate::constants::TWO_KB;
use crate::cartridge::{Cartridge};

pub struct MemoryBus {
    pub ram: RAM,
    pub rom: ROM,
}

impl MemoryBus {
    pub const RESET_VECTOR_ADDR: u16 = 0xFFFC;
    pub const RESET_VECTOR_HIGH_ADDR: u16 = 0xFFFD;
    pub const RESET_VECTOR_DEFAULT: u16 = 0x8000;

    pub fn load_cartridge(cartridge: Cartridge) -> Self {
        Self {
            ram: RAM::new(TWO_KB),
            rom: ROM::new(cartridge.prg_rom),
        }
    }
    
    pub fn read(&self, address: u16) -> u8 {
        if Self::is_ram(address) {
            return self.ram.read(address)
        }
        if Self::is_rom(address) {
            return self.rom.read(address)
        }
        if Self::is_reset_vector(address) {
            return self.read_reset_vector(address)
        }

        // Default value for unmapped regions
        0xFF
    }

    // region: Helper Functions

    #[inline(always)]
    fn is_ram(address: u16) -> bool {
        address <= 0x1FFF
    }

    #[inline(always)]
    fn is_rom(address: u16) -> bool {
        address >= 0x8000 && address <= 0xFFFF
    }

    #[inline(always)]
    fn is_reset_vector(address: u16) -> bool {
        address == 0xFFFC || address == 0xFFFD
    }

    fn read_reset_vector(&self, address: u16) -> u8 {
        let reset_vector = [Self::RESET_VECTOR_DEFAULT as u8, (Self::RESET_VECTOR_DEFAULT >> 8) as u8];
        reset_vector[(address - Self::RESET_VECTOR_ADDR) as usize]
    }

    // endregion: Helper Functions
}
