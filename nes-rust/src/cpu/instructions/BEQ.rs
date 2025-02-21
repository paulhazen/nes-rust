use crate::cpu::status_register::StatusRegister;
use crate::cpu::CPU;
use crate::define_instruction;

define_instruction!(BEQ, |cpu: &mut CPU, memory, offset:u8| {
    let is_zero = cpu.processor_status.is_set(StatusRegister::ZERO);
    cpu.branch(memory, is_zero, offset);
});
