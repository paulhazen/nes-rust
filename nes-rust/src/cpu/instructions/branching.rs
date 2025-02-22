// BEQ, BNE, BCS, BCC, BMI, BPL, BVC, BVS

use crate::cpu::CPU;
use crate::cpu::Status;
use crate::define_instruction;

define_instruction!(BEQ, |cpu: &mut CPU, memory, offset:u8| {
    let is_zero = cpu.is_flag_set(Status::ZERO);
    cpu.branch(memory, is_zero, offset);
});

define_instruction!(BPL, |cpu: &mut CPU, memory, operand : u8| {
    let is_positive = !cpu.is_flag_set(Status::NEGATIVE);
    cpu.branch(memory, is_positive, operand)
});
