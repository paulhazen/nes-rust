
use crate::define_instruction;
use crate::cpu::CPU;

define_instruction!(DEX, |cpu: &mut CPU, _, _ :u8| {
    let new_x = cpu.get_x().wrapping_sub(1);
    cpu.set_x(new_x)
});
