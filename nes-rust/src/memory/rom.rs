pub struct ROM {
    pub data: Box<[u8]>,
    mask: u16,
}

impl ROM {
    pub fn new(prg_rom: Box<[u8]>) -> Self {
        let size = prg_rom.len();
        let mask = match size {
            0x4000 => 0x3FFF, // 16KB ROM (mirrored)
            0x8000 => 0x7FFF, // 32KB ROM
            _ => 0xFFFF,      // Larger ROMs may use bank switching
        };

        Self { data: prg_rom, mask }
    }

    pub fn write(&mut self, address:u16, value: u8) {
        let mapped_addr = (address & self.mask) as usize;
        self.data[mapped_addr] = value;
    }
    
    pub fn read(&self, address: u16) -> u8 {
        let mapped_addr = (address & self.mask) as usize;
        self.data.get(mapped_addr).copied().unwrap_or(0xFF)
    }
}