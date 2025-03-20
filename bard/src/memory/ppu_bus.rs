use std::cell::Cell;
use crate::{cartridge::Cartridge, memory::bus::Bus};

pub struct PPUBus {
    memory: Box<[u8]>,
    oam: [u8; 256],        // Object Attribute Memory (OAM) for sprites
    pub ppu_ctrl: u8,          // $2000 - PPUCTRL
    ppu_mask: u8,          // $2001 - PPUMASK
    ppu_status: u8,        // $2002 - PPUSTATUS
    oam_addr: u8,          // $2003 - OAMADDR
    ppu_scroll: (u8, u8),  // $2005 - PPUSCROLL (x, y)
    ppu_addr: u16,         // $2006 - VRAM Address
    vram_buffer: u8,       // Buffered read for $2007
    last_read_value: Cell<u8>,
    cycle_counter: Cell<u8>,
    nmi_callback: Option<Box<dyn FnMut()>>,
}

impl PPUBus {
    pub fn write_register(&mut self, address: u16, value: u8) {
        match address {
            0x2000 => {
                // PPUCTRL: Control register
                self.ppu_ctrl = value;
                //println!("PPU: Control Register set to {:02X}", value);
            }
            0x2001 => {
                // PPUMASK: Rendering settings
                self.ppu_mask = value;
                //println!("PPU: Mask Register set to {:02X}", value);
            }
            0x2003 => {
                // OAMADDR: Set OAM address
                self.oam_addr = value;
            }
            0x2004 => {
                // OAMDATA: Write to OAM at OAMADDR
                self.oam[self.oam_addr as usize] = value;
                self.oam_addr = self.oam_addr.wrapping_add(1); // Auto-increment
            }
            0x2005 => {
                // PPUSCROLL: Write X/Y scroll position
                if self.ppu_scroll.0 == 0 {
                    self.ppu_scroll.0 = value;
                } else {
                    self.ppu_scroll.1 = value;
                }
            }
            0x2006 => {
                static mut HIGH_BYTE_LATCH: bool = false;
                
                if unsafe { HIGH_BYTE_LATCH } == false {
                    // First write (high byte)
                    self.ppu_addr = ((value as u16) << 8) | (self.ppu_addr & 0x00FF);
                    unsafe { HIGH_BYTE_LATCH = true };
                } else {
                    // Second write (low byte) - full address is now set
                    self.ppu_addr = (self.ppu_addr & 0xFF00) | (value as u16);
                    unsafe { HIGH_BYTE_LATCH = false };
            
                    /* println!(
                        "DEBUG: PPUADDR Fully Set -> Address: 0x{:04X}",
                        self.ppu_addr
                    ); */
                }
            }            
            0x2007 => {
                // Write to VRAM using the existing write_byte function
                self.write_byte(self.ppu_addr, value);
            
/*                 println!(
                    "DEBUG: VRAM Write -> Address: 0x{:04X}, Value: {:02X}",
                    self.ppu_addr, value
                ); */
            
                // Increment VRAM address after the write
                self.ppu_addr = self.ppu_addr.wrapping_add(1);
            }
            
            _ => {}
        }
    }
    
    pub fn read_register(&mut self, address: u16) -> u8 {
        match address {
            0x2002 => {
                // PPUSTATUS: Return status and clear VBlank flag
                let status = self.ppu_status;

                self.ppu_status &= !0x80; // ✅ Clear VBlank flag (bit 7)
                self.nmi_callback = None; // ✅ Prevent unwanted NMIs
                self.oam_addr = 0x00; // ✅ Reset OAM latch

                status
            }
            0x2004 => {
                // OAMDATA: Read from OAM at OAMADDR
                self.oam[self.oam_addr as usize]
            }
            
            0x2007 => {
                // Read from VRAM using the existing read_byte function
                let addr = self.ppu_addr;
                let result = self.vram_buffer;
                self.vram_buffer = self.read_byte(self.ppu_addr); // Fetch next value into buffer
            
                // Increment VRAM address after the read
                self.ppu_addr = self.ppu_addr.wrapping_add(1);
            
/*                 println!(
                    "DEBUG: VRAM Read -> Address: 0x{:04X}, Buffered: {:02X}, Returning: {:02X}",
                    addr, self.vram_buffer, result
                ); */
            
                // Palette reads return actual value instead of buffered
                if addr >= 0x3F00 {
                    self.vram_buffer
                } else {
                    result
                }
            }
            
            _ => 0
        }
    }    

    pub fn set_nmi_callback<F: FnMut() + 'static>(&mut self, callback: F) {
        self.nmi_callback = Some(Box::new(callback));
    }

    pub fn trigger_nmi(&mut self) {
        if let Some(callback) = self.nmi_callback.as_mut() {
            callback();
        }
    }
}

impl Bus for PPUBus {
    fn readable_ranges() -> &'static [std::ops::Range<u16>] {
        &[
            0x0000..0x2000, // Pattern tables (CHR-ROM / CHR-RAM)
            0x2000..0x3000, // Nametables (Mirroring handled separately)
            0x3F00..0x4000, // Palette RAM + Mirrors
        ]
    }
    
    fn writeable_ranges() -> &'static [std::ops::Range<u16>] {
        &[
            0x2000..0x3000, // Nametable VRAM
            0x3F00..0x4000, // Palette RAM + Mirrors
        ]
    }

    fn load_cartridge(cartridge: Cartridge) -> Self {
        let mut memory = vec![0; 0x4000].into_boxed_slice(); // 16 KB for PPU memory
    
        // Load CHR-ROM (or allocate CHR-RAM if missing)
        if !cartridge.chr_rom.is_empty() {
            memory[0x0000..0x2000].copy_from_slice(&cartridge.chr_rom);
        }
    
        // Debugging: Print first 512 bytes of CHR-ROM
        /* for i in 0x0000..0x0200 {
            println!("DEBUG: CHR-ROM Address 0x{:04X} = 0x{:02X}", i, memory[i]);
        } */
    
        // Initialize PPUBus with registers and VRAM initialized
        Self {
            memory,
            oam: [0; 256],   // Sprite memory (OAM)
            ppu_ctrl: 0x00,  // Default value of $2000
            ppu_mask: 0x00,  // Default value of $2001
            ppu_status: 0xA0, // VBlank flag initially set (bit 7 = 1 on startup)
            oam_addr: 0x00,  // Default OAM address
            ppu_scroll: (0, 0), // Scroll X/Y
            ppu_addr: 0x0000, // VRAM Address initially 0
            vram_buffer: 0x00, // Buffered read for $2007
            cycle_counter: Cell::new(0),
            last_read_value: Cell::new(0),
            nmi_callback: None,
        }
    }
    

    fn read_byte(&self, address:u16) -> u8 {
        let value = self.default_read_byte(address);
        value
    }

    fn read_word(&self, address:u16) -> u16 {
        Bus::default_read_word(self, address)
    }
    
    fn write_byte(&mut self, address:u16, value:u8) -> bool {
        self.default_write_byte(address, value)
    }

    fn set_cycle_counter(&self, value: u8) {
        self.cycle_counter.set(value)
    }

    fn get_cycles(&self) -> u8 {
        return self.cycle_counter.get()
    }

    fn set_last_read_value(&self, value: u8) {
        self.last_read_value.set(value)
    }

    fn get_last_read_value(&self) -> u8 {
        self.last_read_value.get()
    }

    fn memory(&self) -> &Box<[u8]> {
        &self.memory
    }

    fn memory_mut(&mut self) -> &mut Box<[u8]> {
        &mut self.memory
    }
    
    fn mask_address(address: u16) -> u16 {
        let masked_address = match address {
            0x3000..=0x3EFF => address - 0x1000, // Nametable mirrors
    
            0x3F20..=0x3FFF => 0x3F00 + (address & 0x1F), // Palette mirrors
    
            0x2000..=0x2FFF => {
                // Nametable mirroring
                let offset = (address - 0x2000) % 0x0800; // Mirror within 2KB of VRAM
                0x2000 + offset
            }
            
            _ => address, // Everything else remains unchanged
        };
    
        masked_address
    }
}