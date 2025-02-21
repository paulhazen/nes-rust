use crate::cpu::status_register::StatusRegister;
use crate::cpu::CPU;
use crate::define_instruction;

define_instruction!(BPL, |cpu: &mut CPU, memory, operand : u8| {
    if !cpu.processor_status.is_set(StatusRegister::NEGATIVE) {
        //cpu.branch(memory, offset);
    }
});

