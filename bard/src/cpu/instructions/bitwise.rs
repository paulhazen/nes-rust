// AND, ORA, EOR, BIT
use crate::cpu::CPU;
use crate::cpu::Status;
use crate::memory::Bus;
use crate::memory::CPUBus;
use crate::define_instruction;

define_instruction!(AND, |cpu: &mut CPU, _: &mut CPUBus, value: u8| {
    cpu.set_a(cpu.get_a() & value);
    cpu.update_zero_and_negative_flags(cpu.get_a());
});

define_instruction!(ORA, |cpu: &mut CPU, _: &mut CPUBus, value: u8| {
    cpu.set_a(cpu.get_a() | value);
    cpu.update_zero_and_negative_flags(cpu.get_a());
});

define_instruction!(EOR, |cpu: &mut CPU, _: &mut CPUBus, value: u8| {
    cpu.set_a(cpu.get_a() ^ value);
    cpu.update_zero_and_negative_flags(cpu.get_a());
});

define_instruction!(BIT, |cpu: &mut CPU, _: &mut CPUBus, value: u8| {
    cpu.set_flag(Status::ZERO, cpu.get_a() & value == 0);
    cpu.set_flag(Status::OVERFLOW, value & 0x40 != 0);
    cpu.set_flag(Status::NEGATIVE, value & 0x80 != 0);
});