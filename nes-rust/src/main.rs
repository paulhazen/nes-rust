mod constants;
pub mod cpu;
pub mod memory;
pub mod macros;

pub use memory::MemoryBus;
pub use cpu::CPU;
use nes_rust::{cartridge::{self}, system::NES};

fn main() {
    let mut nes = NES::open_rom("../roms/dk.nes");
    nes.run();
    println!("Nothing to see here :D")
}
