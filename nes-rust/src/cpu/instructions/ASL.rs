use super::super::Status;
use super::super::CPU;
use crate::memory::MemoryBus;
use crate::define_instruction;

define_instruction!(ASL, |cpu: &mut CPU, _memory: &mut MemoryBus, mut value: u8| {
    cpu.set_flag(Status::CARRY, value & 0x80 != 0);

    value <<= 1; // Shift left


    cpu.set_flag(Status::ZERO, value == 0);

    cpu.update_zero_and_negative_flags(value);
    cpu.set_a(value);
});

