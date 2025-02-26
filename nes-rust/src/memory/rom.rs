use crate::cartridge::Cartridge;
use crate::memory::memory_view::MemoryView;
use crate::memory::MemoryBus;

pub struct ROM {
    start: usize,
    end: usize,
}

impl ROM {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

impl MemoryView for ROM {

    fn write(&mut self, _memory: &mut [u8], _address:u16, _value: u8) {
        // TODO: Probably panic here - ROM should never be written to.
    }
    
    fn read(&self, memory: &[u8], address: u16) -> u8 {
        let index = (address as usize).saturating_sub(self.start);
        if index < (self.end - self.start) {
            memory[self.start + index]
        } else {
            MemoryBus::UNMAPPED
        }
    }
}