use crate::memory::memory_view::MemoryView;

pub struct RAM {
    start: usize,
    mirror_mask: u16
}

impl RAM {
    const DEFAULT_MIRROR_MASK: u16 = 0x07FF;

    pub fn new(start: usize, mirror_mask: u16) -> Self {
        Self { start, mirror_mask }
    }
}

impl MemoryView for RAM {
    fn read(&self, memory: &[u8], address: u16) -> u8 {
        let index = (self.start + (address as usize & self.mirror_mask as usize)) as usize;
        memory[index]
    }

    fn write(&mut self, memory: &mut [u8], address: u16, value: u8) {
        let index = (self.start + (address as usize & self.mirror_mask as usize)) as usize;
        memory[index] = value;
    }
}