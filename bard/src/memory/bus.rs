use std::ops::Range;
use crate::{util, cartridge::Cartridge};

pub trait Bus {

    fn readable_ranges() -> &'static [Range<u16>];

    fn writeable_ranges() -> &'static [Range<u16>];

    fn load_cartridge(cartridge: Cartridge) -> Self;

    fn default_write_byte(&mut self, address: u16, value: u8) -> bool {

        if address == 0x2006 {
            println!("DEBUG: CPU set PPU address to: 0x{:04X}", value as u16);
        }
        if address == 0x2007 {
            println!(
                "DEBUG: CPU wrote 0x{:02X} to PPUDATA (VRAM) at 0x{:04X}",
                value, address // Ensure you have a variable tracking VRAM address
            );
        }

        let masked_address = Self::mask_address(address);

        if !self.is_writeable(address) {
            return false;
        }

        self.memory_mut()[masked_address as usize] = value;
        self.increment_cycle_counter();

        return true;
    }

    fn default_read_word(&self, address: u16) -> u16 {
        let low_byte = self.read_byte(address) as u16;
        let high_byte = self.read_byte(address.wrapping_add(1)) as u16;

        low_byte | (high_byte << 8)
    }

    fn default_read_byte(&self, address: u16) -> u8 {

        // Regardless of if it's open bus or not, the cycle gets incremented.
        self.increment_cycle_counter();

        // Apply the address mask as needed
        let masked_address = Self::mask_address(address);

        // Implement open-bus behavior - where invalid reads return the last
        // byte that was successfully read.
        if !self.is_readable(masked_address) {
            return self.get_last_read_value();
            
        }

        let value = self.memory()[masked_address as usize];
        
        self.set_last_read_value(value);

        return value;
    }

    fn start_cycle_counter(&mut self) {
        self.set_cycle_counter(0)
    }

    fn increment_cycle_counter(&self) {
        let new_cycle_count = self.get_cycles().wrapping_add(1);
        self.set_cycle_counter(new_cycle_count)
    }

    fn is_readable(&self, address:u16) -> bool {
        self.is_valid_address(address) && Self::readable_ranges().iter().any(|r| r.contains(&address))
    }

    fn is_writeable(&self, address:u16) -> bool {
        self.is_valid_address(address) && Self::writeable_ranges().iter().any(|r| r.contains(&address))
    }

    // Note that this function EXPECTS THE ADDRESS to be masked
    fn is_valid_address(&self, address:u16) -> bool {
        return self.memory().len() > address as usize
    }

    fn dump_memory(&self) {
        util::print_hex_dump(self.memory().clone(), None);   
    }

    fn write_byte(&mut self, address:u16, value:u8) -> bool;
    fn read_byte(&self, address:u16) -> u8;
    fn read_word(&self, address: u16) -> u16;
    fn mask_address(address: u16) -> u16;
    fn set_cycle_counter(&self, value: u8);
    fn get_cycles(&self) -> u8;
    fn set_last_read_value(&self, value: u8);
    fn get_last_read_value(&self) -> u8;
    fn memory(&self) -> &Box<[u8]>;
    fn memory_mut(&mut self) -> &mut Box<[u8]>;
}