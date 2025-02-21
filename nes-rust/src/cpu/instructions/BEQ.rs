use crate::cpu::status_register::StatusRegister;
use crate::cpu::CPU;
use crate::define_instruction;

define_instruction!(BEQ, |cpu: &mut CPU, _memory, _:u8| {
    if cpu.processor_status.is_set(StatusRegister::ZERO) {
        //cpu.branch(memory, offset);
    }
});
