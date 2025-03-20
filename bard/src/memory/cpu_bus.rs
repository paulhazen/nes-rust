use crate::{cartridge::Cartridge, memory::bus::Bus};
use std::{cell::{Cell, RefCell}, rc::Rc};

use super::PPUBus;

pub struct CPUBus {
    memory: Box<[u8]>,
    ppu_bus: Option<Rc<RefCell<PPUBus>>>,
    last_read_value: Cell<u8>,
    cycle_counter: Cell<u8>,
}

impl CPUBus {
    pub const UNMAPPED: u8 = 0xFF;
    pub const RESET_VECTOR_ADDR_LOW: u16 = 0xFFFC;
    pub const RESET_VECTOR_ADDR_HIGH: u16 = 0xFFFD;
    pub const RESET_VECTOR_DEFAULT: u16 = 0x8000;
    pub const RAM_START: u16 = 0x0000;
    pub const RAM_END: u16 = 0x1FFF;

    pub fn set_ppu_bus(&mut self, ppu_bus: Rc<RefCell<PPUBus>>) {
        self.ppu_bus = Some(ppu_bus);
    }

    pub fn trigger_nmi(&mut self) {
        println!("CPU: NMI Triggered!");
    }
}

impl Bus for CPUBus {

    fn load_cartridge(cartridge: Cartridge) -> Self {
        let mut memory = vec![Self::UNMAPPED; 0x10000].into_boxed_slice();
    
        let prg_rom_size = cartridge.prg_rom.len();
        let prg_rom_start = Self::RESET_VECTOR_DEFAULT as usize; // PRG-ROM is mapped to $8000-$BFFF
    
        // Copy PRG-ROM into $8000-$BFFF
        memory[prg_rom_start..(prg_rom_start + prg_rom_size)]
            .copy_from_slice(&cartridge.prg_rom);
    
        // If PRG-ROM is only 16KB, mirror it into $C000-$FFFF
        if prg_rom_size == 0x4000 {
            memory[0xC000..=0xFFFF].copy_from_slice(&cartridge.prg_rom);
        }
    
        // Ensure the reset vector is read from the cartridge PRG-ROM
        let reset_vector_lsb = memory[0xFFFC]; // Low byte
        let reset_vector_msb = memory[0xFFFD]; // High byte
        let reset_address = ((reset_vector_msb as u16) << 8) | (reset_vector_lsb as u16);
    
        println!("Loaded reset vector: {:04X}", reset_address); // Debugging
    
        Self {
            memory,
            ppu_bus: None,
            last_read_value: Cell::new(Self::UNMAPPED),
            cycle_counter: Cell::new(0x00),
        }
    }
    

    fn mask_address(address: u16) -> u16 {
        match address {
            0x0000..0x1FFF => {
                // This address is in the NES part of the RAM 
                // that is mirrored to 0x0000-0x07FF, therefore the
                // address needs to be masked to 2KB
                address & 0x07FF
            }
            0x2008..0x3FFF => {
                // This address is in the NES part of the PPU registers
                // that are mirrored to 0x2000-0x2007, therefore the 
                // address needs to be masked appropriately to that 
                // range
                0x2000 + (address & 0x0007)
            }
            _ => {
                // The region of memory being accessed does not need
                // to be masked - it maps directly to the address it
                // already is
                address
            }
        }
    }

    fn write_byte(&mut self, address: u16, value:u8) -> bool {
        if (0x2000..=0x2007).contains(&address) {
            if let Some(bus) = &self.ppu_bus {
                bus.borrow_mut().write_register(address, value);
                return true;
            }
        }
        Bus::default_write_byte(self, address, value)
    }

    fn read_byte(&self, address: u16) -> u8 {
        if (0x2000..=0x2007).contains(&address) {
            if let Some(bus) = &self.ppu_bus {
                return bus.borrow_mut().read_register(address)
            }
        }
        Bus::default_read_byte(self, address)
    }

    fn read_word(&self, address: u16) -> u16 {
        Bus::default_read_word(self, address)
    }

    fn readable_ranges() -> &'static [std::ops::Range<u16>] {
        &[Self::RAM_START..0xFFFF]
    }

    fn writeable_ranges() -> &'static [std::ops::Range<u16>] {
        &[Self::RAM_START..Self::RAM_END]
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

    fn memory(&self) -> &Box<[u8]> {
        &self.memory
    }

    fn memory_mut(&mut self) -> &mut Box<[u8]> {
        &mut self.memory
    }
}