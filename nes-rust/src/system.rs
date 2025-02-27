use crate::cpu::CPU;
use crate::memory::MemoryBus;
use crate::cartridge::Cartridge;

pub struct NES {
    pub cpu: CPU,
    pub memory_bus: MemoryBus,
}

impl NES {
    pub fn open_rom(rom_filepath: &str) -> Self {
        let cartridge = Cartridge::load_from_file(rom_filepath).unwrap();
        let memory_bus = MemoryBus::load_cartridge(cartridge);
        let cpu = CPU::new();

        cpu.dbg_view_opcode_table();

        Self {cpu, memory_bus }
    }

    pub fn run(&mut self) {

        // Reset the CPU explicitly before running (TODO: This may not be needed)
        self.cpu.reset(&self.memory_bus);

        // Print the reset vector
        self.memory_bus.debug_view_reset_vector();

        loop {
            self.cpu.step(&mut self.memory_bus);
        }
    }
}