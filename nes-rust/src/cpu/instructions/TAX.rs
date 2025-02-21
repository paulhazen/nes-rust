use crate::define_instruction;
use crate::cpu::CPU;

define_instruction!(TAX, |cpu: &mut CPU, _, _ :u8| {
    cpu.set_x_register(cpu.get_accumulator());
});