pub mod cpu;
pub mod memory;
pub mod util;
pub mod cartridge;
pub use cpu::CPU;
use nes_rust::nes::NES;

fn main() {
    let mut nes = NES::open_rom("../roms/smb.nes");
    nes.run();
    println!("Nothing to see here :D")
}
