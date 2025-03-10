pub mod cpu;
pub mod memory;
pub mod util;
pub mod cartridge;
pub use cpu::CPU;
use bard::nes::NES;

fn main() {
    let mut nes = NES::open_rom("../roms/smb.nes");
    nes.run();
}
