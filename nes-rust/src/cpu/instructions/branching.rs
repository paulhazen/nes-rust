// BEQ, BNE, BCS, BCC, BMI, BPL, BVC, BVS

use crate::cpu::CPU;
use crate::cpu::Status;
use crate::memory::Bus;
use crate::define_instruction;

define_instruction!(BEQ, |cpu: &mut CPU, memory, offset: u8| {
    let is_zero = cpu.is_flag_set(Status::ZERO);
    cpu.branch(memory, is_zero, offset);
});

define_instruction!(BNE, |cpu: &mut CPU, memory, offset: u8| {
    let is_not_zero = !cpu.is_flag_set(Status::ZERO);
    cpu.branch(memory, is_not_zero, offset);
});

define_instruction!(BCS, |cpu: &mut CPU, memory, offset: u8| {
    let is_carry_set = cpu.is_flag_set(Status::CARRY);
    cpu.branch(memory, is_carry_set, offset);
});

define_instruction!(BCC, |cpu: &mut CPU, memory, offset: u8| {
    let is_carry_clear = !cpu.is_flag_set(Status::CARRY);
    cpu.branch(memory, is_carry_clear, offset);
});

define_instruction!(BMI, |cpu: &mut CPU, memory, offset: u8| {
    let is_negative = cpu.is_flag_set(Status::NEGATIVE);
    cpu.branch(memory, is_negative, offset);
});

define_instruction!(BPL, |cpu: &mut CPU, memory, offset: u8| {
    let is_positive = !cpu.is_flag_set(Status::NEGATIVE);
    cpu.branch(memory, is_positive, offset);
});

define_instruction!(BVC, |cpu: &mut CPU, memory, offset: u8| {
    let is_overflow_clear = !cpu.is_flag_set(Status::OVERFLOW);
    cpu.branch(memory, is_overflow_clear, offset);
});

define_instruction!(BVS, |cpu: &mut CPU, memory, offset: u8| {
    let is_overflow_set = cpu.is_flag_set(Status::OVERFLOW);
    cpu.branch(memory, is_overflow_set, offset);
});
