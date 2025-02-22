use crate::define_instruction;
use crate::cpu::CPU;

define_instruction!(TXA, |cpu: &mut CPU, _, _ :u8| {
    cpu.set_a(cpu.get_x());
});

define_instruction!(TYA, |cpu: &mut CPU, _, _ :u8| {
    cpu.set_a(cpu.get_y());
});

define_instruction!(TAY, |cpu: &mut CPU, _, _ :u8| {
    cpu.set_y(cpu.get_a());
});

define_instruction!(TAX, |cpu: &mut CPU, _, _ :u8| {
    cpu.set_x(cpu.get_a());
});