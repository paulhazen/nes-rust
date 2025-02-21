use nes_rust::system::NES;
mod common;
use std::time::Duration;
use std::thread;
use std::sync::mpsc;

fn run_system_test()
{
    let rom_path = common::setup_test_rom("dk.nes");

    let mut nes = NES::open_rom(rom_path.to_str().unwrap());

    nes.run();
}

#[test]
fn test_execute_instruction() {
    use std::time::Duration;
    use std::thread;

    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        run_system_test();
        let _ = tx.send(());
    });

    if rx.recv_timeout(Duration::from_secs(5)).is_err() {
        panic!("Test timed out! Possible infinite loop in emulator.");
    }

    let _ = handle.join();
}