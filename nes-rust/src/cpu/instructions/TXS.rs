use crate::define_instruction;
use crate::cpu::CPU;

define_instruction!(TXS, |cpu: &mut CPU, _, _ :u8| {
    cpu.set_s(cpu.get_x());
});
