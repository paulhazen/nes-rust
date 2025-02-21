use nes_rust::memory::MemoryBus;
use nes_rust::cartridge::Cartridge;
use nes_rust::cartridge::CartridgeHeader;

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper function to create a test cartridge with specified PRG-ROM data.
    fn create_test_cartridge(prg_rom_data: Vec<u8>) -> Cartridge {
        Cartridge {
            header: CartridgeHeader {
                prg_rom_size: (prg_rom_data.len() / 16_384) as u8, // Calculate size in 16KB units
                chr_rom_size: 0, // No CHR-ROM needed for CPU memory tests
                mapper_id: 0, // NROM (No mapper)
            },
            prg_rom: prg_rom_data.into_boxed_slice(),
            chr_rom: vec![].into_boxed_slice(), // Empty CHR-ROM
        }
    }

    #[test]
    fn test_ram_write_and_read() {
        let cartridge = create_test_cartridge(vec![0; 16 * 1024]); // Empty PRG-ROM (16KB)
        let mut bus = MemoryBus::load_cartridge(cartridge);

        // Write to RAM at $0000
        bus.write(0x0000, 0x42);
        assert_eq!(bus.read(0x0000), 0x42);

        // Test mirroring ($0000 - $07FF mirrors up to $1FFF)
        assert_eq!(bus.read(0x0800), 0x42);
        assert_eq!(bus.read(0x1000), 0x42);
        assert_eq!(bus.read(0x1800), 0x42);

        // Write at another mirrored location and check all mirrors
        bus.write(0x07FF, 0x99);
        assert_eq!(bus.read(0x07FF), 0x99);
        assert_eq!(bus.read(0x0FFF), 0x99);
        assert_eq!(bus.read(0x17FF), 0x99);
    }

    #[test]
    fn test_prg_rom_read_16kb_mirror() {
        let prg_rom_data = vec![0xAA; 16 * 1024]; // 16KB ROM filled with 0xAA
        let cartridge = create_test_cartridge(prg_rom_data);
        let bus = MemoryBus::load_cartridge(cartridge);

        // Read from PRG-ROM ($8000-$BFFF) - Should be 0xAA
        assert_eq!(bus.read(0x8000), 0xAA);
        assert_eq!(bus.read(0x9000), 0xAA);
        assert_eq!(bus.read(0xBFFF), 0xAA);

        // Read from PRG-ROM mirrored region ($C000-$FFFF) - Should also be 0xAA
        assert_eq!(bus.read(0xC000), 0xAA);
        assert_eq!(bus.read(0xD000), 0xAA);
        assert_eq!(bus.read(0xFFFF), 0xAA);
    }

    #[test]
    fn test_prg_rom_read_32kb() {
        let prg_rom_data = vec![0x55; 32 * 1024]; // 32KB ROM filled with 0x55
        let cartridge = create_test_cartridge(prg_rom_data);
        let bus = MemoryBus::load_cartridge(cartridge);

        // Read from first 16KB bank ($8000-$BFFF) - Should be 0x55
        assert_eq!(bus.read(0x8000), 0x55);
        assert_eq!(bus.read(0x9000), 0x55);
        assert_eq!(bus.read(0xBFFF), 0x55);

        // Read from second 16KB bank ($C000-$FFFF) - Should be 0x55 (not mirrored)
        assert_eq!(bus.read(0xC000), 0x55);
        assert_eq!(bus.read(0xD000), 0x55);
        assert_eq!(bus.read(0xFFFF), 0x55);
    }

    #[test]
    fn test_rom_write_is_ignored() {
        let prg_rom_data = vec![0xBB; 16 * 1024]; // 16KB ROM filled with 0xBB
        let cartridge = create_test_cartridge(prg_rom_data);
        let mut bus = MemoryBus::load_cartridge(cartridge);

        // Attempt to write to ROM
        bus.write(0x8000, 0x42);
        bus.write(0x9000, 0x13);
        bus.write(0xBFFF, 0x77);

        // ROM should still be read-only (original values should be unchanged)
        assert_eq!(bus.read(0x8000), 0xBB);
        assert_eq!(bus.read(0x9000), 0xBB);
        assert_eq!(bus.read(0xBFFF), 0xBB);
    }

    #[test]
    fn test_unmapped_memory_returns_ff() {
        let cartridge = create_test_cartridge(vec![0; 16 * 1024]); // Empty PRG-ROM (16KB)
        let bus = MemoryBus::load_cartridge(cartridge);

        // Addresses outside RAM/ROM should return 0xFF
        assert_eq!(bus.read(0x5000), 0xFF); // Expansion ROM (unused)
        assert_eq!(bus.read(0x6000), 0xFF); // SRAM (not enabled in NROM)
        assert_eq!(bus.read(0x7000), 0xFF); // Unmapped memory
        assert_eq!(bus.read(0x401F), 0xFF); // APU/IO (not handled yet)
    }
}
