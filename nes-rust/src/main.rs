pub mod cpu;
pub mod memory;
<<<<<<< HEAD
pub mod util;
=======
pub mod macros;

>>>>>>> ac64b2fa59787330904af5315794eab496bbd747
pub use cpu::CPU;
use nes_rust::{cartridge::{self}, system::NES};

fn main() {
<<<<<<< HEAD
    let mut nes = NES::open_rom("../roms/smb.nes");
=======
    let mut nes = NES::open_rom("../roms/dk.nes");
>>>>>>> ac64b2fa59787330904af5315794eab496bbd747
    nes.run();
    println!("Nothing to see here :D")
}
