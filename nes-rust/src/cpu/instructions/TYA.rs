use crate::cpu::CPU;
use crate::define_instruction;

define_instruction!(TYA, |cpu: &mut CPU, _, _ :u8| {
    cpu.set_accumulator(cpu.get_y_register());
});
