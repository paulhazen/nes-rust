use crate::define_instruction;
use crate::cpu::CPU;

define_instruction!(TAY, |cpu: &mut CPU, _, _ :u8| {
    cpu.set_y(cpu.get_a());
});