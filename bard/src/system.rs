use crate::cpu::CPU;
use crate::memory::Bus;
use crate::memory::CPUBus;
use crate::cartridge::Cartridge;

pub struct NES {
    pub cpu: CPU,
    pub memory_bus: CPUBus,
}

impl NES {
    pub fn open_rom(rom_filepath: &str) -> Self {
        let cartridge = Cartridge::load_from_file(rom_filepath).unwrap();
        let memory_bus = CPUBus::load_cartridge(cartridge);

        memory_bus.dump_memory();

        let cpu = CPU::new();

        cpu.dbg_view_opcode_table();

        Self {cpu, memory_bus }
    }

    pub fn run(&mut self) {

        // Reset the CPU explicitly before running (TODO: This may not be needed)
        self.cpu.reset(&self.memory_bus);

        loop {
            self.cpu.step(&mut self.memory_bus);
        }
    }
}