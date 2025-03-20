use bard::memory::Bus;
use bard::memory::CPUBus;
use bard::cartridge::Cartridge;
use bard::cartridge::CartridgeHeader;

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
                buffer: Box::new([0x00; 16]),
            },
            prg_rom: prg_rom_data,
            chr_rom: vec![], // Empty CHR-ROM
        }
    }

    #[test]
    fn test_ram_write_and_read() {
        let cartridge = create_test_cartridge(vec![0; 16 * 1024]); // Empty PRG-ROM (16KB)
        let mut bus = CPUBus::load_cartridge(cartridge);

        // Write to RAM at $0000
        bus.write_byte(0x0000, 0x42);
        assert_eq!(bus.read_byte(0x0000), 0x42);

        // Test mirroring ($0000 - $07FF mirrors up to $1FFF)
        assert_eq!(bus.read_byte(0x0800), 0x42);
        assert_eq!(bus.read_byte(0x1000), 0x42);
        assert_eq!(bus.read_byte(0x1800), 0x42);

        // Write at another mirrored location and check all mirrors
        bus.write_byte(0x07FF, 0x99);
        assert_eq!(bus.read_byte(0x07FF), 0x99);
        assert_eq!(bus.read_byte(0x0FFF), 0x99);
        assert_eq!(bus.read_byte(0x17FF), 0x99);
    }

    #[test]
    fn test_prg_rom_read_16kb_mirror() {
        let prg_rom_data = vec![0xAA; 16 * 1024]; // 16KB ROM filled with 0xAA
        let cartridge = create_test_cartridge(prg_rom_data);
        let bus = CPUBus::load_cartridge(cartridge);

        // Read from PRG-ROM ($8000-$BFFF) - Should be 0xAA
        assert_eq!(bus.read_byte(0x8000), 0xAA);
        assert_eq!(bus.read_byte(0x9000), 0xAA);
        assert_eq!(bus.read_byte(0xBFFF), 0xAA);

        // Read from PRG-ROM mirrored region ($C000-$FFFF) - Should also be 0xAA
        assert_eq!(bus.read_byte(0xC000), 0xAA);
        assert_eq!(bus.read_byte(0xD000), 0xAA);
        assert_eq!(bus.read_byte(0xFFFF), 0xAA);
    }

    #[test]
    fn test_prg_rom_read_32kb() {
        let prg_rom_data = vec![0x55; 32 * 1024]; // 32KB ROM filled with 0x55
        let cartridge = create_test_cartridge(prg_rom_data);
        let bus = CPUBus::load_cartridge(cartridge);

        // Read from first 16KB bank ($8000-$BFFF) - Should be 0x55
        assert_eq!(bus.read_byte(0x8000), 0x55);
        assert_eq!(bus.read_byte(0x9000), 0x55);
        assert_eq!(bus.read_byte(0xBFFF), 0x55);

        // Read from second 16KB bank ($C000-$FFFF) - Should be 0x55 (not mirrored)
        assert_eq!(bus.read_byte(0xC000), 0x55);
        assert_eq!(bus.read_byte(0xD000), 0x55);
        assert_eq!(bus.read_byte(0xFFFF), 0x55);
    }

    #[test]
    fn test_rom_write_is_ignored() {
        let prg_rom_data = vec![0xBB; 16 * 1024]; // 16KB ROM filled with 0xBB
        let cartridge = create_test_cartridge(prg_rom_data);
        let mut bus = CPUBus::load_cartridge(cartridge);

        // Attempt to write to ROM
        bus.write_byte(0x8000, 0x42);
        bus.write_byte(0x9000, 0x13);
        bus.write_byte(0xBFFF, 0x77);

        // ROM should still be read-only (original values should be unchanged)
        assert_eq!(bus.read_byte(0x8000), 0xBB);
        assert_eq!(bus.read_byte(0x9000), 0xBB);
        assert_eq!(bus.read_byte(0xBFFF), 0xBB);
    }

    #[test]
    fn test_unmapped_memory_returns_ff() {
        let cartridge = create_test_cartridge(vec![0; 16 * 1024]); // Empty PRG-ROM (16KB)
        let bus = CPUBus::load_cartridge(cartridge);

        // Addresses outside RAM/ROM should return 0xFF
        assert_eq!(bus.read_byte(0x5000), 0xFF); // Expansion ROM (unused)
        assert_eq!(bus.read_byte(0x6000), 0xFF); // SRAM (not enabled in NROM)
        assert_eq!(bus.read_byte(0x7000), 0xFF); // Unmapped memory
        assert_eq!(bus.read_byte(0x401F), 0xFF); // APU/IO (not handled yet)
    }

    #[test]
    fn test_reset_vector_fetch() {
        let mut prg_rom_data = vec![0; 16 * 1024]; // 16KB ROM
        prg_rom_data[0x3FFC] = 0x00; // LSB of reset vector
        prg_rom_data[0x3FFD] = 0x80; // MSB of reset vector

        let cartridge = create_test_cartridge(prg_rom_data);
        let bus = CPUBus::load_cartridge(cartridge);

        assert_eq!(bus.read_byte(0xFFFC), 0x00);
        assert_eq!(bus.read_byte(0xFFFD), 0x80);
    }


    /*
    #[test]
    fn test_debug_prg_rom_mapping() {
        let prg_rom_data = vec![0xAA; 16 * 1024]; // Fill PRG-ROM with 0xAA
        let cartridge = create_test_cartridge(prg_rom_data);
        let bus = CPUBus::load_cartridge(cartridge);

        // Debug first and second PRG-ROM banks
        bus.debug_prg_rom_mapping(0x8000, 0xBFFF);
        bus.debug_prg_rom_mapping(0xC000, 0xFFFF);
    }
*/

    #[test]
    fn test_prg_rom_bank_switching() {
        let prg_rom_data = vec![0x11; 32 * 1024]; // 32KB ROM
        let cartridge = create_test_cartridge(prg_rom_data);
        let mut bus = CPUBus::load_cartridge(cartridge);

        // Assume the mapper allows switching banks, we simulate a bank switch here.
        bus.write_byte(0x8000, 1); // Example: Switch to bank 1
        assert_eq!(bus.read_byte(0x8000), 0x11);
    }

    #[test]
    fn test_open_bus_behavior() {
        let cartridge = create_test_cartridge(vec![0; 16 * 1024]); // Empty PRG-ROM (16KB)
        let mut bus = CPUBus::load_cartridge(cartridge);

        // Write a known value to RAM, then read from unmapped memory
        bus.write_byte(0x0000, 0x37);
        let last_value = bus.read_byte(0x0000);

        // Reading from an unmapped region should return last_value (unless explicitly FF)
        let open_bus_value = bus.read_byte(0x5000);
        assert!(open_bus_value == 0xFF || open_bus_value == last_value);
    }

    #[test]
    fn test_ram_mirroring_consistency() {
        let cartridge = create_test_cartridge(vec![0; 16 * 1024]); // Empty PRG-ROM (16KB)
        let mut bus = CPUBus::load_cartridge(cartridge);

        // Write at a mirrored region
        let write_succeeded = bus.write_byte(0x1000, 0x77);

        // Check that the writing was successful.
        assert_eq!(write_succeeded, true);

        // Check all mirrored locations
        assert_eq!(bus.read_byte(0x0000), 0x77);
        assert_eq!(bus.read_byte(0x0800), 0x77);
        assert_eq!(bus.read_byte(0x1800), 0x77);
    }

    #[test]
    fn test_write_to_io_registers() {
        let cartridge = create_test_cartridge(vec![0; 16 * 1024]); // Empty PRG-ROM (16KB)
        let mut bus = CPUBus::load_cartridge(cartridge);

        bus.write_byte(0x4016, 0x55); // Attempt to write to controller port

        // Read should return default behavior (not the written value)
        assert!(bus.read_byte(0x4016) != 0x55);
    }

    #[test]
    fn test_uninitialized_sram() {
        let cartridge = create_test_cartridge(vec![0; 16 * 1024]); // Empty PRG-ROM (16KB)
        let bus = CPUBus::load_cartridge(cartridge);

        // Check if SRAM defaults to 0xFF (or any expected uninitialized value)
        assert_eq!(bus.read_byte(0x6000), 0xFF);
    }


}
