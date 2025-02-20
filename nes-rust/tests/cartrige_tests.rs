mod common; // Import the helper
use nes_rust::cartridge::Cartridge;

use std::fs::File;
use std::io::{Read, Result};

fn load_rom(rom_path: &str) -> Result<Vec<u8>> {
    let mut file = File::open(rom_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

#[test]
fn test_rom_loading() {
    let rom_path = common::setup_test_rom("dk.nes");
    
    match load_rom(rom_path.to_str().unwrap()) {
        Ok(data) => assert!(!data.is_empty(), "ROM should not be empty"),
        Err(e) => panic!("Failed to load test ROM: {}", e),
    }

    // Try loading the cartridge
    let cartridge = Cartridge::load_from_file(rom_path.to_str().unwrap()).ok().unwrap();

    assert_eq!(cartridge.header.mapper_id, 0, "Mapper ID for Donkey Kong should be 0.");
    println!("TEST");
}

