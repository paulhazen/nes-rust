use std::time::Duration;

use crate::cpu::CPU;
use crate::ppu::PPU;
use crate::memory::MemoryBus;
use crate::cartridge::Cartridge;
use crate::framebuffer_viewer::FramebufferViewer;
use crate::memory::CPUBus;
use std::thread;
use crate::memory::Bus;

pub struct NES {

    pub cpu: CPU,
    pub ppu: PPU,

    pub cpu_bus: CPUBus,
    pub memory_bus: MemoryBus,
    pub viewer: FramebufferViewer,
}

impl NES {
    pub fn open_rom(rom_filepath: &str) -> Self {
        let cartridge = Cartridge::load_from_file(rom_filepath).unwrap();
        let memory_bus = MemoryBus::load_cartridge(cartridge.clone());
        let cpu_bus = CPUBus::load_cartridge(cartridge.clone());
        let cpu = CPU::new();
        let ppu = PPU::load_from_cartridge(&cartridge.clone());
        let viewer = FramebufferViewer::new();

        cpu.dbg_view_opcode_table();

        Self {
            cpu, 
            ppu, 
            cpu_bus,
            memory_bus, 
            viewer 
        }
    }

    pub fn run(&mut self) {

        // Reset the CPU explicitly before running (TODO: This may not be needed)
        self.cpu.reset(&self.memory_bus);

        //let rng = rand::rng();

        loop {
            //for pixel in self.ppu.frame_buffer.iter_mut() {
            //    *pixel = rng.random_range(0..=3);
            //}

            let cycles = self.cpu.tick(&mut self.memory_bus);

            for _ in 0..(cycles * 3) {
                self.ppu.tick()
            }

            self.viewer.update(&self.ppu.frame_buffer);

            thread::sleep(Duration::from_millis(16));

            if !self.viewer.is_open() {
                break
            }
        }
    }
}