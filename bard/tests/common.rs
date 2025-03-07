use std::fs;
use std::path::{Path, PathBuf};

use std::fs::File;
use std::io::{Read, Result};
use bard::Cartridge;

fn load_rom(rom_path: &str) -> Result<Vec<u8>> {
    let mut file = File::open(rom_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

pub fn load_test_rom(rom_filename: &str) -> Cartridge {
    let rom_path = setup_test_rom("dk.nes");
    
    match load_rom(rom_path.to_str().unwrap()) {
        Ok(data) => assert!(!data.is_empty(), "ROM should not be empty"),
        Err(e) => panic!("Failed to load test ROM: {}", e),
    }

    // Try loading the cartridge
    Cartridge::load_from_file(rom_path.to_str().unwrap()).ok().unwrap()
}

/// Copies a test ROM to the target directory for testing.
pub fn setup_test_rom(rom_filename: &str) -> PathBuf {
    let out_dir = std::env::var("CARGO_TARGET_DIR")
        .unwrap_or_else(|_| "target".to_string()); // Default to `target/`
    let dest_dir = Path::new(&out_dir).join("test_roms");

    fs::create_dir_all(&dest_dir).expect("Failed to create test ROM directory");

    let src_path = Path::new("../roms").join(rom_filename);
    let dest_path = dest_dir.join(rom_filename);

    fs::copy(&src_path, &dest_path).expect("Failed to copy ROM file for testing");

    dest_path
}
