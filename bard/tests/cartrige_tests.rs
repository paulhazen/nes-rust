mod common; 
use bard::Cartridge;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{Write, Seek, SeekFrom};
    use bard::Cartridge;
    use tempfile::tempdir;

    /// Helper function to create a temporary NES ROM file.
    fn create_nes_rom(prg_size: usize, chr_size: usize) -> (tempfile::TempDir, String) {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test_rom.nes");
        let mut file = File::create(&file_path).unwrap();

        // NES header (16 bytes)
        let mut header = vec![0; 16];
        header[0..3].copy_from_slice(b"NES"); // NES magic bytes
        header[3] = 0x1A; // NES format byte
        header[4] = (prg_size / 16_384) as u8; // PRG size in 16KB units
        header[5] = (chr_size / 8_192) as u8; // CHR size in 8KB units
        header[6] = 0x00; // Mapper ID (lower 4 bits)
        file.write_all(&header).unwrap();

        // PRG-ROM Data (filled with 0xAA)
        file.write_all(&vec![0xAA; prg_size]).unwrap();

        // CHR-ROM Data (filled with 0xBB, unless CHR-ROM is 0, indicating CHR-RAM)
        if chr_size > 0 {
            file.write_all(&vec![0xBB; chr_size]).unwrap();
        }

        (dir, file_path.to_string_lossy().to_string())
    }

    #[test]
    fn test_load_valid_cartridge() {
        let (_dir, file_path) = create_nes_rom(32_768, 8_192); // 32KB PRG, 8KB CHR

        let cartridge = Cartridge::load_from_file(&file_path);
        assert!(cartridge.is_ok(), "Failed to load a valid NES ROM");

        let cartridge = cartridge.unwrap();
        assert_eq!(cartridge.prg_rom.len(), 32_768);
        assert_eq!(cartridge.chr_rom.len(), 8_192);
        assert_eq!(cartridge.header.prg_rom_size, 2); // 2 * 16KB
        assert_eq!(cartridge.header.chr_rom_size, 1); // 1 * 8KB
    }

    #[test]
    fn test_load_invalid_nes_header() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("invalid_rom.nes");
        let mut file = File::create(&file_path).unwrap();

        // Write incorrect header (missing "NES" magic bytes)
        file.write_all(&[0x00; 16]).unwrap();

        let cartridge = Cartridge::load_from_file(file_path.to_str().unwrap());
        assert!(cartridge.is_err(), "Should fail due to invalid NES header");
    }

    #[test]
    fn test_read_prg_rom_mirroring() {
        let (_dir, file_path) = create_nes_rom(16_384, 8_192); // 16KB PRG, 8KB CHR
        let cartridge = Cartridge::load_from_file(&file_path).unwrap();

        // Test reading from PRG-ROM (mirroring for 16KB ROMs)
        assert_eq!(cartridge.read_prg_rom(0x8000), 0xAA);
        assert_eq!(cartridge.read_prg_rom(0xBFFF), 0xAA);
        assert_eq!(cartridge.read_prg_rom(0xC000), 0xAA);
        assert_eq!(cartridge.read_prg_rom(0xFFFF), 0xAA);
    }

    #[test]
    fn test_read_prg_rom_out_of_bounds() {
        let (_dir, file_path) = create_nes_rom(32_768, 8_192); // 32KB PRG, 8KB CHR
        let cartridge = Cartridge::load_from_file(&file_path).unwrap();

        // Test addresses outside valid PRG-ROM range
        assert_eq!(cartridge.read_prg_rom(0x0000), 0xFF);
        assert_eq!(cartridge.read_prg_rom(0x7FFF), 0xFF);
    }

    #[test]
    fn test_chr_rom_extraction() {
        let (_dir, file_path) = create_nes_rom(32_768, 8_192);
        let cartridge = Cartridge::load_from_file(&file_path).unwrap();

        // Verify CHR-ROM extraction
        let chr_rom = cartridge.get_chr_rom();
        assert_eq!(chr_rom.len(), 8_192);
        assert_eq!(chr_rom[0], 0xBB);
        assert_eq!(chr_rom[8_191], 0xBB);
    }
}

