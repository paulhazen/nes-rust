pub const KB: usize = 1024;
pub const TWO_KB: usize = 2 * KB;

// A NROM (with no Mapper) is 32KB in size
pub const NROM_SIZE : usize = 32 * KB;

pub const NES_HEADER_START: [u8; 3] = [0x4E, 0x45, 0x53];