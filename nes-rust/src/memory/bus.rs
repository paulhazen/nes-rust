use std::ops::Range;
use crate::cartridge::Cartridge;

pub trait Bus {

    fn readable_ranges() -> &'static [Range<u16>];

    fn writeable_ranges() -> &'static [Range<u16>];

    fn mirror_mask() -> u16;

    fn load_cartridge(cartridge: Cartridge) -> Self;

    fn write_byte(&mut self, address: u16, value: u8) -> bool {
        if !self.is_writeable(address) {
            return false;
        }

        self.memory_mut()[address as usize] = value;
        self.increment_cycle_counter();

        return true;
    }

    fn read_word(&self, address: u16) -> u16 {
        let low_byte = self.read_byte(address) as u16;
        let high_byte = self.read_byte(address.wrapping_add(1)) as u16;

        (high_byte << 8) | low_byte
    }

    fn read_byte(&self, address: u16) -> u8 {
        let masked_address = address & Self::mirror_mask();
        // Implement open-bus behavior - where invalid reads return the last
        // byte that was successfully read.
        if !self.is_readable(masked_address) {
            self.get_last_read_value();
        }

        let value = self.memory()[masked_address as usize];
        self.increment_cycle_counter();

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

    fn is_valid_address(&self, address:u16) -> bool {
        return self.memory().len() > address as usize
    }

    fn set_cycle_counter(&self, value: u8);
    fn get_cycles(&self) -> u8;
    fn set_last_read_value(&self, value: u8);
    fn get_last_read_value(&self) -> u8;
    fn memory(&self) -> &[u8];
    fn memory_mut(&mut self) -> &mut [u8];
}