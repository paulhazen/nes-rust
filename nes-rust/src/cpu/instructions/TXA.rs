use crate::define_instruction;
use crate::cpu::CPU;

define_instruction!(TXA, |cpu: &mut CPU, _, _ :u8| {
    cpu.set_a(cpu.get_x());
});
