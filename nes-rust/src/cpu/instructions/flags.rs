// CLC, SEC, CLI, SEI, CLV, CLD, SED

use crate::cpu::CPU;
use crate::cpu::Status;
use crate::memory::Bus;
use crate::define_instruction;

define_instruction!(CLC, |cpu: &mut CPU, _, _: u8| {
    cpu.set_flag(Status::CARRY, false);
});

define_instruction!(SEC, |cpu: &mut CPU, _, _: u8| {
    cpu.set_flag(Status::CARRY, true);
});

define_instruction!(CLI, |cpu: &mut CPU, _, _: u8| {
    cpu.set_flag(Status::INTERRUPT_DISABLE, false);
});

define_instruction!(SEI, |cpu: &mut CPU, _, _: u8| {
    cpu.set_flag(Status::INTERRUPT_DISABLE, false)
});

define_instruction!(CLV, |cpu: &mut CPU, _, _: u8| {
    cpu.set_flag(Status::OVERFLOW, false);
});

define_instruction!(CLD, |cpu: &mut CPU, _, _ :u8| {
    cpu.set_flag(Status::DECIMAL, false)
});

define_instruction!(SED, |cpu: &mut CPU, _, _: u8| {
    cpu.set_flag(Status::DECIMAL, true)
});