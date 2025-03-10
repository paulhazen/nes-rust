//! # nes.rs
//!
//!  Author: Paul Hazen
//! Created: 2025-03-10
//! License: MIT (see LICENSE file)
//!
//! ## Description
//! Contains the implementation for the NES struct - which serves to orchestrate the various components of the emulator.
//! 
use std::{cell::RefCell, rc::Rc};
use crate::cpu::CPU;
use crate::ppu::PPU;
use crate::cartridge::Cartridge;
use crate::framebuffer_viewer::FramebufferViewer;
use crate::memory::CPUBus;
use crate::memory::PPUBus;
use crate::memory::Bus;

pub struct NES {

    pub cpu: CPU,
    pub cpu_bus: CPUBus,

    pub ppu: PPU,
    pub ppu_bus: Rc<RefCell<PPUBus>>,
    
    pub viewer: FramebufferViewer,
}

impl NES {
    fn dump_nametable(ppu_bus: &PPUBus) {
        println!("=== Nametable Dump (0x2000 - 0x23BF) ===");
    
        for tile_y in 0..30 {
            for tile_x in 0..32 {
                let address = 0x2000 + (tile_y * 32 + tile_x) as u16;
                let tile_index = ppu_bus.read_byte(address);
                print!("{:02X} ", tile_index);
            }
            println!(); // New line after each row
        }
    }
    
    pub fn open_rom(rom_filepath: &str) -> Self {
        let cartridge = Cartridge::load_from_file(rom_filepath).unwrap();

        let cpu = CPU::new();
        
        let ppu = PPU::load_from_cartridge(&cartridge); // Create PPU first
        let ppu_bus = Rc::new(RefCell::new(PPUBus::load_cartridge(cartridge.clone()))); // Create PPU bus

        // Set VRAM address to 0x2000
        ppu_bus.borrow_mut().write_register(0x2006, 0x20);  
        ppu_bus.borrow_mut().write_register(0x2006, 0x00);  

        // First read (ignored value from buffer)
        let _ = ppu_bus.borrow_mut().read_register(0x2007);

        // Second read (actual VRAM content)
        let read_value = ppu_bus.borrow_mut().read_register(0x2007);

        println!("DEBUG: Read VRAM value = {:02X}", read_value);

        Self::dump_nametable(&ppu_bus.borrow_mut());

        //ppu.print_chr_rom_tiles(&ppu_bus.borrow());

        let mut cpu_bus = CPUBus::load_cartridge(cartridge);
        cpu_bus.set_ppu_bus(Rc::clone(&ppu_bus));

        //ppu_bus.borrow_mut().set_nmi_callback(|| cpu_bus.trigger_nmi());

        let viewer = FramebufferViewer::new(&rom_filepath);

        Self {
            cpu,
            cpu_bus,
            ppu,
            ppu_bus,
            viewer,
        }
    }
    

    pub fn run(&mut self) {

        // Reset the CPU explicitly before running (TODO: This may not be needed)
        self.cpu.reset(&self.cpu_bus);

        // Dump the nametable after execution
        Self::dump_nametable(&self.ppu_bus.borrow());

       loop {
            let _cycles = self.cpu.step(&mut self.cpu_bus);

            for _ in 0..3 {
                self.ppu.tick(&mut self.ppu_bus.borrow_mut());
            }

            self.viewer.update(&self.ppu.frame_buffer);

            if !self.viewer.is_open() {
                break
            }
        }
    }
}