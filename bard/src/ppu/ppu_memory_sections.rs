/* use std::ops::Range;

pub enum PpuMemorySections {
    PatternTable,
    NameTable,
    Palette,
    Unmapped,
}

impl PpuMemorySections {

    const PATTERN_TABLE_RANGE: Range<u8> = (0x0000..0x1FFF);
    const NAMETABLE_RANGE: Range<u8> = (0x2000..0x2FFF);
    const MIRRORED_NAMETABLE_RANGE: Range<u8> = (0x3000..0x2EFF);
    const PALETTE_RANGE: Range<u8> = (0x3F00..0x3FFF);

    fn get_section(address: &mut u8) -> PpuMemorySections {
        address = address & PPU::ADDRESS_MASK;

        match address {
            Self::PATTERN_TABLE_RANGE => {
                PpuMemorySections::PatternTable
            }
            Self::NAME_TABLE_BASE_ADDRESS => {
                PpuMemorySections::NameTable
            }
            Self::MIRRORED_NAMETABLE_RANGE => {
                // for mirrored nametable also shift the address
                address = address - 0x1000;
                PpuMemorySections::NameTable
            }
            Self::PALETTE_RANGE => {
                PpuMemorySections::Palette
            }
            _ => {
                PpuMemorySections::Unmapped
            }
        }
    }
} */