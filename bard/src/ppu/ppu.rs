use crate::{cartridge::Cartridge, memory::{Bus, PPUBus}};

// TODO: Move these constants into PPU if possible.
const PPU_FRAME_BUFFER_HEIGHT: usize = 240;
const PPU_FRAME_BUFFER_WIDTH: usize = 256;
const PPU_CYCLES_PER_SCANLINE: u16 = 341;
const PPU_VISIBLE_SCANLINES: u16 = 240; // Scanlines where pixels are drawn
const PPU_POST_RENDER_SCANLINE: u16 = 240; // Idle scanline
const PPU_VBLANK_START_SCANLINE: u16 = 241; // VBlank begins
const PPU_VBLANK_END_SCANLINE: u16 = 260; // Last VBlank scanline
const PPU_PRE_RENDER_SCANLINE: u16 = 261; // Prepares for next frame
const PPU_TOTAL_SCANLINES: u16 = 262; // Total scanlines per frame
const STATUS_VBLANK_FLAG: u8 = 0b1000_0000; // Bit 7 in PPUSTATUS ($2002)
const CONTROL_NMI_ENABLE_FLAG: u8 = 0b1000_0000; // Bit 7 in PPUCTRL ($2000)
const CHR_ROM_SIZE: usize = 8192; // 8 KB of CHR-ROM (Pattern Table Data)

pub struct PPU {
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
}

impl PPU {

    const ATTRIBUTE_TABLE_BASE_ADDRESS: u16 = 0x23C0;
    const PATTERN_TABLE_BASE_ADDRESS: u16 = 0x0000;
    const NAME_TABLE_BASE_ADDRESS: u16 = 0x2000;
    const PALETTE_BASE_ADDRESS: u16 = 0x3F00;
    pub const ADDRESS_MASK: u16 = 0x3FFF;

    pub fn load_from_cartridge(cartridge: &Cartridge) -> Self {
        let mut chr_rom = [0u8; CHR_ROM_SIZE];
        let cartridge_chr = cartridge.get_chr_rom();

        let copy_size = std::cmp::min(CHR_ROM_SIZE, cartridge_chr.len());
        chr_rom[..copy_size].copy_from_slice(&cartridge_chr[..copy_size]);

        PPU {
            frame_buffer: [0x00; PPU_FRAME_BUFFER_WIDTH * PPU_FRAME_BUFFER_HEIGHT],   // Initialize frame buffer to empty
            cycle: 0,                                                                 // Start at the first PPU cycle
            scanline: PPU_PRE_RENDER_SCANLINE,                                        // Pre-render scanline
            frame_count: 0,                                                           // First frame has not started
            nmi_triggered: false,                                                     // No pending NMI interrupt
            control_register: 0x00,                                                   // All bits start cleared
            status_register: 0xA,
        }
    }

    pub fn tick(&mut self, ppu_bus: &mut PPUBus, cpu_cycles: u8) {
        for _ in 0..(cpu_cycles * 3) { // Each CPU cycle advances the PPU by ~3
            self.cycle += 1;
    
            if self.scanline < PPU_VISIBLE_SCANLINES && self.cycle < PPU_FRAME_BUFFER_WIDTH as u16 {
                self.render_pixel(ppu_bus);
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
    }
    
    
    fn render_pixel(&mut self, ppu_bus: &mut PPUBus) {
        let x = self.cycle as usize;
        let y = self.scanline as usize;
    
        if x >= PPU_FRAME_BUFFER_WIDTH || y >= PPU_FRAME_BUFFER_HEIGHT {
            return; // Skip if out of bounds
        }
    
        let color = self.fetch_background_pixel(ppu_bus, x, y);
        self.frame_buffer[y * PPU_FRAME_BUFFER_WIDTH + x] = color;
    }
    

    fn fetch_background_pixel(&self, ppu_bus: &PPUBus, x: usize, y: usize) -> u8 {
        // Get tile index from nametable
        let tile_index = self.read_nametable(ppu_bus, x, y);
    
        // Get tile pixel data from CHR-ROM
        let tile_data = self.read_pattern_table(ppu_bus, tile_index, y % 8);
    
        // Get color palette from attribute table
        let color_palette = self.read_attribute_table(x, y, ppu_bus);
    
        // Extract the correct pixel from the tile row
        let pixel = (tile_data >> (7 - (x % 8))) & 1;

        // Convert to correct color
        let color = self.get_final_pixel_color(ppu_bus, pixel, color_palette); 

        color
    } 
    
    fn read_attribute_table(&self, x: usize, y: usize, ppu_bus: &PPUBus) -> u8 {
        let tile_x = x / 16; // 16x16 pixel block
        let tile_y = y / 16;
    
        let attribute_address = Self::ATTRIBUTE_TABLE_BASE_ADDRESS + ((tile_y * 8) + tile_x) as u16;
    
        let attribute_byte = ppu_bus.read_byte(attribute_address);
    
        // Extract the correct 2-bit palette for this 8x8 tile
        let quadrant_x = (x % 16) / 8;
        let quadrant_y = (y % 16) / 8;
        let shift = (quadrant_y * 2 + quadrant_x) * 2;
    
        (attribute_byte >> shift) & 0b11 // Extract the 2-bit palette index
    }
    
    fn get_final_pixel_color(&self, ppu_bus: &PPUBus, pixel: u8, color_palette: u8) -> u8 {
        if pixel == 0 {
            return 0; // Color 0 is transparent
        } 
    
        let color_index = Self::PALETTE_BASE_ADDRESS + ((color_palette as u16 * 4) + pixel as u16) as u16;
        let color = ppu_bus.read_byte(color_index);
    
        color
    }
    
    fn read_pattern_table(&self, ppu_bus: &PPUBus, tile_index: u8, row: usize) -> u8 {
        let pattern_table_base = if self.control_register & 0b0001_0000 != 0 {
            0x1000 // Pattern Table 1
        } else {
            0x0000 // Pattern Table 0
        };
    
        let tile_address = pattern_table_base + (tile_index as u16 * 16) + row as u16;
        let low_byte = ppu_bus.read_byte(tile_address);
        let high_byte = ppu_bus.read_byte(tile_address + 8);
    
        low_byte | (high_byte << 1) // This is how NES forms a 2-bit color index
    }
    
    fn read_nametable(&self, ppu_bus: &PPUBus, x: usize, y: usize) -> u8 {
        let tile_x = x / 8;
        let tile_y = y / 8;
    
        let base_address = ((tile_y * 32) + tile_x) as u16;
        let tile_index = ppu_bus.read_byte(base_address);
    
        tile_index
    }
    
    
    fn read_palette(&self, ppu_bus: &PPUBus, address: u16) -> u8 {
        let mirrored_address = match address {
            0x3F10 | 0x3F14 | 0x3F18 | 0x3F1C => address - 0x10, // Mirror sprite palettes
            _ => address,
        };
    
        ppu_bus.memory()[mirrored_address as usize]
    }

    fn enter_vblank(&mut self) {
        self.status_register |= 0x80;
        
        if self.control_register & 0x80 != 0 {
            self.nmi_triggered = true;
        }
    }
    

    fn start_new_frame(&mut self) {
        self.scanline = 0;
        self.frame_count += 1;
    
        // Clear VBlank flag only if it hasn't been cleared by CPU read
        if self.status_register & STATUS_VBLANK_FLAG != 0 {
            self.status_register &= !STATUS_VBLANK_FLAG;
        }
    }
    
    
    #[cfg(debug_assertions)]
    pub fn print_chr_rom_tiles(&self, ppu_bus: &PPUBus) {
        const TILE_SIZE: usize = 16;
        let num_tiles = CHR_ROM_SIZE / TILE_SIZE;
        let tiles_per_row = 16; // CHR-ROM typically has 16 tiles per row
    
        for tile_index in 0..num_tiles {
            let tile_x = tile_index % tiles_per_row;
            let tile_y = tile_index / tiles_per_row;
    
            self.print_tile(ppu_bus, tile_x, tile_y);
        }
    }
    
    fn print_tile(&self, ppu_bus: &PPUBus, tile_x: usize, tile_y: usize) {
        const TILE_SIZE: usize = 16;
        let tiles_per_row = 16;
        
        // Compute tile index from (x, y) position
        let tile_index = tile_y * tiles_per_row + tile_x;
    
        for row in 0..8 {
            let tile_address = tile_index * TILE_SIZE + row;
            let low_byte = ppu_bus.memory()[tile_address];
            let high_byte = ppu_bus.memory()[tile_address + 8];
    
            let mut row_str = String::new();
    
            for col in 0..8 {
                let bit_position = 7 - col;
                let low_bit = (low_byte >> bit_position) & 1;
                let high_bit = (high_byte >> bit_position) & 1;
                let pixel_value = (high_bit << 1) | low_bit;
    
                let symbol = match pixel_value {
                    0 => '.',
                    1 => '░',
                    2 => '▒',
                    3 => '▓',
                    _ => '?',
                };
    
                row_str.push(symbol);
            }
    
            println!("{}", row_str);
        }
    
        println!();
    }
    
    
}
