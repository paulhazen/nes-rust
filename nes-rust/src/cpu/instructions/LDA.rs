use crate::cpu::CPU;
use crate::define_instruction;

define_instruction!(LDA, |cpu: &mut CPU, _, value| {
    cpu.set_a(value);
    cpu.update_zero_and_negative_flags(value);
}); 
