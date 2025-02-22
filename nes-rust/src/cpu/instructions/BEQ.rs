use crate::cpu::CPU;
use crate::cpu::Status;
use crate::define_instruction;

define_instruction!(BEQ, |cpu: &mut CPU, memory, offset:u8| {
    let is_zero = cpu.is_flag_set(Status::ZERO);
    cpu.branch(memory, is_zero, offset);
});
