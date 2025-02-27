<<<<<<< HEAD

use crate::cartridge::Cartridge;

const PPU_CYCLES_PER_SCANLINE: u16 = 341;
const PPU_FRAME_BUFFER_HEIGHT: usize = 240;
const PPU_FRAME_BUFFER_WIDTH: usize = 256;
const PPU_VISIBLE_SCANLINES: u16 = 240; // Scanlines where pixels are drawn
const PPU_POST_RENDER_SCANLINE: u16 = 240; // Idle scanline
const PPU_VBLANK_START_SCANLINE: u16 = 241; // VBlank begins
const PPU_VBLANK_END_SCANLINE: u16 = 260; // Last VBlank scanline
const PPU_PRE_RENDER_SCANLINE: u16 = 261; // Prepares for next frame
const PPU_TOTAL_SCANLINES: u16 = 262; // Total scanlines per frame
const STATUS_VBLANK_FLAG: u8 = 0b1000_0000; // Bit 7 in PPUSTATUS ($2002)
const CONTROL_NMI_ENABLE_FLAG: u8 = 0b1000_0000; // Bit 7 in PPUCTRL ($2000)
const CHR_ROM_SIZE: usize = 8192; // 8 KB of CHR-ROM (Pattern Table Data)

pub struct PPU{
    pub frame_buffer: [u8; PPU_FRAME_BUFFER_WIDTH * PPU_FRAME_BUFFER_HEIGHT],
    cycle: u16,
    scanline: u16,

    /*    
        ------------------------------------------------------------------------------------------
        | Bit | Name                         | Function                                          |
        ------------------------------------------------------------------------------------------
        | 1-0 | Base Nametable Address       | Controls scrolling area ($2000-$2C00)             |
        |  2  | VRAM Address Increment       | 0 = increment by 1, 1 = increment by 32           |
        |  3  | Pattern Table for Background | 0 = $0000, 1 = $1000                              |
        |  4  | Pattern Table for Sprites    | 0 = $0000, 1 = $1000                              |
        |  5  | Sprite Size                  | 0 = 8x8, 1 = 8x16                                 |
        |  6  | PPU Master/Slave Mode        | Unused on NES                                     |
        |  7  | VBlank NMI Enable            | Triggers an NMI when entering VBlank (1 = enable) |
        ------------------------------------------------------------------------------------------
     */
    control_register: u8,

    /*
        Indicates whether an NMI (Non-Maskable Interrupt) should be triggered.
     */
    nmi_triggered: bool,

    /*
        -------------------------------------------------------------------------------------------------------------------
        | Bit | Name                                       | Function                                                     |
        -------------------------------------------------------------------------------------------------------------------
        | 4-0 | Unused (returns stale PPU open bus values) |                                                              |
        |  5  | Sprite Overflow                            | Set if too many sprites are on one scanline                  |
        |  6  | Sprit 0 Hit                                | Set when sprite 0 collides with background pixels            |
        |  7  | VBlank Flag                                | Set to 1 when entering VBlank, cleared when CPU reads $2002  |
        -------------------------------------------------------------------------------------------------------------------
     */
    status_register: u8,
    frame_count: u64,
    vram: [u8; 2048], // PPU has 2KB of VRAM

    palette_ram: [u8; 32], // 32 bytes of palette RAM

    chr_rom: [u8; CHR_ROM_SIZE]
}

impl PPU {
    pub fn load_from_cartridge(cartridge: &Cartridge) -> Self {
        let mut chr_rom = [0u8; CHR_ROM_SIZE];
        let cartridge_chr = cartridge.get_chr_rom();

        let copy_size = std::cmp::min(CHR_ROM_SIZE, cartridge_chr.len());
        chr_rom[..copy_size].copy_from_slice(&cartridge_chr[..copy_size]);

        // Load a simple default NES palette
        let palette_ram = [
            0x0F, 0x30, 0x27, 0x1A, // Background colors
            0x0F, 0x16, 0x20, 0x27, // First sprite palette
            0x0F, 0x16, 0x20, 0x27, // Second sprite palette
            0x0F, 0x16, 0x20, 0x27, // Third sprite palette
            0x0F, 0x16, 0x20, 0x27, // Fourth sprite palette
            0x0F, 0x16, 0x20, 0x27, // Repeated palette
            0x0F, 0x16, 0x20, 0x27,
            0x0F, 0x16, 0x20, 0x27,
        ];

        PPU {
            frame_buffer: [0x00; PPU_FRAME_BUFFER_WIDTH * PPU_FRAME_BUFFER_HEIGHT],   // Initialize frame buffer to empty
            cycle: 0,                                                                 // Start at the first PPU cycle
            scanline: PPU_PRE_RENDER_SCANLINE,                                        // Pre-render scanline
            frame_count: 0,                                                           // First frame has not started
            nmi_triggered: false,                                                     // No pending NMI interrupt
            control_register: 0x00,                                                   // All bits start cleared
            status_register: 0xA,                                                     // Expected open-bus state on power-up
            vram: [0x00; 2048],
            palette_ram: palette_ram,//[0x00; 32],
            chr_rom,
        }
    }

    pub fn tick(&mut self) {
        self.cycle += 1;

        if self.scanline < PPU_VISIBLE_SCANLINES && self.cycle < PPU_FRAME_BUFFER_WIDTH as u16 {
            self.render_pixel();
        }

        if self.cycle >= PPU_CYCLES_PER_SCANLINE {
            self.cycle = 0;
            self.scanline += 1;

            match self.scanline {
                PPU_VBLANK_START_SCANLINE => self.enter_vblank(),
                PPU_TOTAL_SCANLINES => self.start_new_frame(),
                _ => {}
            }
        }
    }

    fn render_pixel(&mut self) {
        let x = self.cycle as usize;
        let y = self.scanline as usize;
    
        if x >= PPU_FRAME_BUFFER_WIDTH || y >= PPU_FRAME_BUFFER_HEIGHT {
            return; // Skip if out of bounds
        }
    
        let color = self.fetch_background_pixel(x, y);
        self.frame_buffer[y * PPU_FRAME_BUFFER_WIDTH + x] = color;
    }
    

    fn fetch_background_pixel(&self, x: usize, y: usize) -> u8 {
        let tile_index = self.read_nametable(x, y);  // Get tile index from nametable
        let tile_data = self.read_pattern_table(tile_index, y % 8); // Get tile pixel data
        let color_palette = self.read_attribute_table(x, y); // Get color palette
    
        let pixel = (tile_data >> (7 - (x % 8))) & 1; // Extract correct pixel from the tile row
    
        self.get_final_pixel_color(pixel, color_palette) // Convert to correct color
    }
    

    fn read_nametable(&self, x: usize, y: usize) -> u8 {
        let tile_x = x / 8;
        let tile_y = y / 8;
        let nametable_base = 0x2000;
        let nametable_address = nametable_base + (tile_y * 32) + tile_x;
        
        let tile_index = self.read_ppu_memory(nametable_address as u16);
        
        if tile_index == 0 {
            println!("DEBUG: Nametable returned tile 0 at ({}, {})", tile_x, tile_y);
        } else {
            println!("DEBUG: Nametable tile index at ({}, {}) = {}", tile_x, tile_y, tile_index);
        }
        
        tile_index
    }
    
    fn read_pattern_table(&self, tile_index: u8, row: usize) -> u8 {
        let pattern_table_base = 0x0000; // Background tiles usually use pattern table 0
        let tile_address = pattern_table_base + (tile_index as u16 * 16) + row as u16;
    
        let low_byte = self.read_ppu_memory(tile_address);         // Low bitplane
        let high_byte = self.read_ppu_memory(tile_address + 8);   // High bitplane
    
        let combined = (high_byte << 1) | low_byte;
    
        println!(
            "DEBUG: Pattern table for tile {} row {}: low=0x{:02X}, high=0x{:02X}, combined=0x{:02X}",
            tile_index, row, low_byte, high_byte, combined
        );
    
        combined
    }
    
    fn read_attribute_table(&self, x: usize, y: usize) -> u8 {
        let tile_x = x / 16; // 16x16 pixel block
        let tile_y = y / 16;
    
        let attribute_table_base = 0x23C0; // Base address for attribute table
        let attribute_address = attribute_table_base + (tile_y * 8) + tile_x;
    
        let attribute_byte = self.read_ppu_memory(attribute_address as u16);
    
        // Extract the correct 2-bit palette for this 8x8 tile
        let quadrant_x = (x % 16) / 8;
        let quadrant_y = (y % 16) / 8;
        let shift = (quadrant_y * 2 + quadrant_x) * 2;
    
        (attribute_byte >> shift) & 0b11 // Extract the 2-bit palette index
    }
    
    
    fn get_final_pixel_color(&self, pixel: u8, color_palette: u8) -> u8 {
        if pixel == 0 {
            println!("DEBUG: Transparent pixel detected");
            return 0; // Color 0 is transparent
        }
    
        let palette_base = 0x3F00; // Base address for palette RAM
        let color_index = palette_base + (color_palette as u16 * 4) + pixel as u16;
        let color = self.read_ppu_memory(color_index);
    
        println!(
            "DEBUG: Pixel color lookup: pixel={} color_palette={} color_index=0x{:04X} -> color=0x{:02X}",
            pixel, color_palette, color_index, color
        );
    
        color
    }
    
    

    fn read_ppu_memory(&self, address: u16) -> u8 {
        let address = address & 0x3FFF; // PPU addresses wrap at 16KB

        match address {
            0x0000..=0x1FFF => {
                // Pattern Table (CHR-ROM or CHR-RAM from the cartridge)
                self.read_pattern_table_memory(address)
            }
            0x2000..=0x2FFF => {
                // Nametable region (may be mirrored)
                self.read_nametable_memory(address)
            }
            0x3000..=0x3EFF => {
                // Mirrors `$2000-$2EFF`
                self.read_nametable_memory(address - 0x1000)
            }
            0x3F00..=0x3FFF => {
                // Palette RAM (with mirroring)
                self.read_palette_memory(address)
            }
            _ => {
                // Unused memory (should return open bus behavior)
                0
            }
        }
    }

    fn read_pattern_table_memory(&self, address: u16) -> u8 {
        let index = address as usize;
    
        if index < CHR_ROM_SIZE {
            self.chr_rom[index] // Read from fixed array
        } else {
            0 // Prevent out-of-bounds access
        }
    }

    fn read_nametable_memory(&self, address: u16) -> u8 {
        let vram_index = (address - 0x2000) as usize % 2048; // Only 2KB VRAM
        self.vram[vram_index]
    }

    fn read_palette_memory(&self, address: u16) -> u8 {
        let mirrored_address = match address {
            0x3F10 | 0x3F14 | 0x3F18 | 0x3F1C => address - 0x10, // Mirror sprite palettes
            _ => address,
        };
    
        self.palette_ram[(mirrored_address - 0x3F00) as usize]
    }

    fn enter_vblank(&mut self) {
        self.status_register |= STATUS_VBLANK_FLAG; // Set VBlank flag in PPUSTATUS

        if self.control_register & CONTROL_NMI_ENABLE_FLAG != 0 {
            self.nmi_triggered = true; // Signal NMI to CPU if enabled
        }
    }

    fn start_new_frame(&mut self) {
        self.scanline = 0;
        self.frame_count += 1;
        self.status_register &= !STATUS_VBLANK_FLAG; // Clear VBlank flag
    }
}
=======
>>>>>>> parent of 9ef4987 (feat: Added PPU support)
