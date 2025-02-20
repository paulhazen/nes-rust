use nes_rust::system::NES;
mod common;

#[test]
fn test_execute_instruction() {
    let rom_path = common::setup_test_rom("dk.nes");

    let mut nes = NES::open_rom(rom_path.to_str().unwrap());

    nes.run();
}