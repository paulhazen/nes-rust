use nes_rust::cpu::CPU;
use nes_rust::memory::MemoryBus;
use nes_rust::cpu::status_register::StatusRegister;
use nes_rust::cpu;
#[test]
fn test_view_opcode_table() {

}

/*
#[test]
fn test_cpu_reset_sequence() {
    let mut memory_bus = MemoryBus::new(vec![0; 0x8000]);
    memory_bus.rom.write(0xFFFC, 0x00);
    memory_bus.rom.write(0xFFFD, 0x80);

    let mut cpu = CPU::new(&memory_bus);
    cpu.reset();

    assert_eq!(cpu.get_program_counter(), 0x8000, "CPU did not reset to the correct address.");
    assert!(cpu.processor_status.is_set(StatusRegister::UNUSED), "UNUSED bit should be set after reset.");
}
    */