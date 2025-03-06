use std::fs;
use std::path::{Path, PathBuf};

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
