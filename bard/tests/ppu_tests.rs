use bard::memory::PPUBus;
use bard::memory::Bus;
use bard::Cartridge;
mod common;

#[test]
fn test_nametable_read_write() {
    let cartridge = common::load_test_rom("dk.nes");
    let mut ppu_bus = PPUBus::load_cartridge(cartridge);

    let test_addr = 0x2005; // Example address in nametable range
    let test_value = 0xAB; // Arbitrary test value

    // Write to the nametable
    ppu_bus.write_byte(test_addr, test_value);

    // Read back the value
    let read_value = ppu_bus.read_byte(test_addr);

    assert_eq!(read_value, test_value, "Nametable read did not match written value");
}

#[test]
fn test_nametable_mirroring() {
    let cartridge = common::load_test_rom("dk.nes");
    let mut ppu_bus = PPUBus::load_cartridge(cartridge);

    let base_addr = 0x2000; // Base nametable start
    let mirror_addr = 0x2800; // Mirrored region

    let test_value = 0x55;

    ppu_bus.write_byte(base_addr, test_value);
    assert_eq!(
        ppu_bus.read_byte(mirror_addr),
        test_value,
        "Nametable mirroring failed"
    );
}

#[test]
fn test_ppu_bus_read_write_full_range() {
    let cartridge = common::load_test_rom("dk.nes");
    let mut ppu_bus = PPUBus::load_cartridge(cartridge);

    // Test writing and reading across all writable ranges
    for range in PPUBus::writeable_ranges() {
        for addr in range.clone() {
            let test_value = (addr & 0xFF) as u8; // Unique test value per address
            
            ppu_bus.write_byte(addr, test_value);
            let read_value = ppu_bus.read_byte(addr);

            assert_eq!(
                read_value, test_value,
                "Mismatch at address 0x{:04X}: expected 0x{:02X}, got 0x{:02X}",
                addr, test_value, read_value
            );
        }
    }

    // Test that readable ranges return values without crashing
    for range in PPUBus::readable_ranges() {
        for addr in range.clone() {
            let _ = ppu_bus.read_byte(addr); // Just ensure it doesn't panic
        }
    }
}
