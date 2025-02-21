use crate::cpu::status_register::StatusRegister;
use crate::cpu::CPU;
use crate::memory::MemoryBus;
use crate::define_instruction;

define_instruction!(ASL, |cpu: &mut CPU, _memory: &mut MemoryBus, mut value: u8| {
    if value & 0x80 != 0 {
        cpu.processor_status.set(StatusRegister::CARRY);
    }
    
    value <<= 1; // Shift left

    if value == 0 {
        cpu.processor_status.set(StatusRegister::ZERO)
    }

    cpu.update_zero_and_negative_flags(value);
    cpu.set_accumulator(value);
});

