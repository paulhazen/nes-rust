use crate::cpu::CPU;
use crate::define_instruction;

define_instruction!(LDX, |cpu: &mut CPU, _, value| {
    cpu.set_x_register(value);
});
