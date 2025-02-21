
use crate::define_instruction;
use crate::cpu::CPU;

define_instruction!(DEX, |cpu: &mut CPU, _, _ :u8| {
    let new_x = cpu.get_x_register().wrapping_sub(1);
    cpu.set_x_register(new_x)
});
