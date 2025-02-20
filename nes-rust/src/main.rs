mod constants;
pub mod cpu;
pub mod memory;
pub mod macros;

pub use memory::MemoryBus;
pub use cpu::CPU;
use nes_rust::cartridge::{self, Cartridge};

fn main() {
    println!("Nothing to see here :D")
}
