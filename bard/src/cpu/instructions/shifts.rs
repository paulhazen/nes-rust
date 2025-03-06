// ASL, LSR, ROL, ROR

use super::super::Status;
use super::super::CPU;
use crate::memory::Bus;
use crate::memory::CPUBus;
use crate::define_instruction;

// Arithmetic Shift Left (ASL) - already implemented
define_instruction!(ASL, |cpu: &mut CPU, _memory: &mut CPUBus, mut value: u8| {
    cpu.set_flag(Status::CARRY, value & 0x80 != 0);

    value <<= 1; // Shift left

    cpu.update_zero_and_negative_flags(value);
    cpu.set_a(value);
});

// Logical Shift Right (LSR)
define_instruction!(LSR, |cpu: &mut CPU, _memory: &mut CPUBus, mut value: u8| {
    cpu.set_flag(Status::CARRY, value & 0x01 != 0);

    value >>= 1; // Shift right

    cpu.update_zero_and_negative_flags(value);
    cpu.set_a(value);
});

// Rotate Left (ROL)
define_instruction!(ROL, |cpu: &mut CPU, _memory: &mut CPUBus, mut value: u8| {
    let carry_in = cpu.is_flag_set(Status::CARRY) as u8;
    let new_carry = value & 0x80 != 0;

    value = (value << 1) | carry_in; // Shift left and insert carry

    cpu.set_flag(Status::CARRY, new_carry);
    cpu.update_zero_and_negative_flags(value);
    cpu.set_a(value);
});

// Rotate Right (ROR)
define_instruction!(ROR, |cpu: &mut CPU, _memory: &mut CPUBus, mut value: u8| {
    let carry_in = (cpu.is_flag_set(Status::CARRY) as u8) << 7;
    let new_carry = value & 0x01 != 0;

    value = (value >> 1) | carry_in; // Shift right and insert carry

    cpu.set_flag(Status::CARRY, new_carry);
    cpu.update_zero_and_negative_flags(value);
    cpu.set_a(value);
});
