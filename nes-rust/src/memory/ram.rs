pub struct RAM {
    data: Vec<u8>,
    mask: u16
}

impl RAM {
    const DEFAULT_MIRROR_MASK: u16 = 0x07FF;

    pub fn new(size:usize) -> Self {
        Self {
            data: vec![0; size],
            mask: Self::DEFAULT_MIRROR_MASK,
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        let mapped_addr = (address & self.mask) as usize;
        self.data[mapped_addr]
    }

    pub fn write(&mut self, address:u16, value: u8) {
        let mapped_addr = (address & self.mask) as usize;
        self.data[mapped_addr] = value;
    }
}