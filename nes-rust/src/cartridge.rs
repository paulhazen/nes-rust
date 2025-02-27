use std::fs::File;
use std::io::{self, BufReader, Read};

use crate::util;

/// Represents an NES cartridge.
pub struct Cartridge {
    /// The parsed NES cartridge header containing metadata.
    pub header: CartridgeHeader,
    /// The program (PRG) ROM data, stored in a boxed slice.
    pub prg_rom: Box<[u8]>,
    /// The character (CHR) ROM data, stored in a boxed slice.
    pub chr_rom: Box<[u8]>,
}

/// Stores metadata from an NES cartridge header.
pub struct CartridgeHeader {
    /// The size of the PRG ROM in 16 KB units.
    pub prg_rom_size: u8,
    /// The size of the CHR ROM in 8 KB units.
    pub chr_rom_size: u8,
    /// The mapper ID, extracted from the header.
    pub mapper_id: u8,
    /// The raw contents of the header component of the cartridge.
    pub buffer: Box<[u8]>,
}

impl Cartridge {
    const NES_HEADER_START: [u8; 3] = [0x4E, 0x45, 0x53];


    fn read_file_to_boxed_bytes(path: &str) -> io::Result<Box<[u8]>> {
        let mut file = File::open(path)?;
        
        // Preallocate buffer with file size (if available)
        let metadata = file.metadata()?;
        let mut buffer = Vec::with_capacity(metadata.len() as usize);

        print!("Buffer is {} items long.", metadata.len());
        file.read_to_end(&mut buffer)?; // Read all bytes

        let blah = buffer.clone().into_boxed_slice();
        //util::print_hex_dump(blah, None);

        Ok(buffer.into_boxed_slice())
    }



    /// Loads an NES cartridge from a specified file path.
    ///
    /// # Arguments
    ///
    /// * `file_path` - A string slice representing the path to the NES ROM file.
    ///
    /// # Returns
    ///
    /// * `Ok(Cartridge)` if the file is successfully read and parsed.
    /// * `Err(io::Error)` if there is a problem reading the file.
    pub fn load_from_file(file_path: &str) -> io::Result<Self> {
        // Open the NES ROM file
        let file = File::open(file_path)?;
        let mut reader = BufReader::new(file);

        let raw_rom = Self::read_file_to_boxed_bytes(file_path)?;
        //util::print_hex_dump(raw_rom, None);

        // Read and validate the NES header
        let header = Self::load_header(&mut reader)?; // ✅ Now propagates `io::Error`

        // Allocate memory for PRG and CHR ROMs based on header values
        let mut prg_rom = vec![0u8; (header.prg_rom_size as usize * 16_384).into()].into_boxed_slice();
        let mut chr_rom = vec![0u8; (header.chr_rom_size as usize * 8_192).into()].into_boxed_slice();

        // Read PRG ROM data
        reader.read_exact(&mut prg_rom)?;

        // Read CHR ROM data (only if non-zero)
        if !chr_rom.is_empty() {
            reader.read_exact(&mut chr_rom)?;
        }

        // Return the loaded Cartridge
        Ok(Self {
            header,
            prg_rom,
            chr_rom,
        })
    }

    /// Reads a byte from PRG ROM
    /// 
    /// # Arguments
    /// 
    /// * `address` - The address from the cpu to read from the PRG-ROM
    /// 
    /// # Returns
    /// 
    /// * `u8` The byte at the given address.
    pub fn read_prg_rom(&self, address: u16) -> u8 {
        if address < 0x8000 {
            return 0xFF; // Outside valid PRG-ROM range
        }
    
        let prg_rom_size = self.prg_rom.len();
        let prg_rom_offset = (address - 0x8000) as usize;
    
        if prg_rom_size == 0x4000 {
            // 16KB ROM - Mirror it to $C000-$FFFF
            self.prg_rom[prg_rom_offset % 0x4000]
        } else if prg_rom_size == 0x8000 {
            // 32KB ROM - Directly map it
            self.prg_rom[prg_rom_offset]
        } else {
            0xFF // Invalid read
        }
    }

    /// Reads and validates the NES file header.
    ///
    /// # Arguments
    ///
    /// * `reader` - A mutable reference to a `BufReader<File>` that provides access to the ROM file.
    ///
    /// # Returns
    ///
    /// * `Ok(CartridgeHeader)` containing the extracted PRG size, CHR size, and mapper ID.
    /// * `Err(io::Error)` if the file header is invalid or unreadable.
    fn load_header(reader: &mut BufReader<File>) -> io::Result<CartridgeHeader> {
        let mut buffer = [0u8; 16];

        // Attempt to read the first 16 bytes (header)
        reader.read_exact(&mut buffer)?;

        // Validate NES header signature
        if !buffer.starts_with(&Cartridge::NES_HEADER_START) {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Not a valid NES file."));
        } else {
            println!("INFO: File has a valid HEADER intro.");
        }

        let boxed_buffer = Box::new(buffer);
        util::print_hex_dump(boxed_buffer.clone(), None);

        // Extract header information
        Ok(CartridgeHeader {
            prg_rom_size: buffer[4],  // PRG ROM size in 16KB units
            chr_rom_size: buffer[5],  // CHR ROM size in 8KB units
            mapper_id: buffer[6],     // Mapper Id
            buffer: boxed_buffer, // The raw buffer representing the header.
        })
    }
}
