mod constants;
pub mod cpu;
pub mod memory;
pub mod macros;

pub use memory::MemoryBus;
pub use cpu::CPU;

fn main() {
    let prg_rom = vec![0x00; 0x8000]; // Example 32KB ROM
    let memory_bus = MemoryBus::new(prg_rom); // Memory is owned here
    let cpu = CPU::new(&memory_bus); // CPU borrows memory

    let value = cpu.read_memory(0x8000);
    println!("Read from 0x8000: {:02X}", value);
}
