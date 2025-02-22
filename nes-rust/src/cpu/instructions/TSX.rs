use crate::define_instruction;
use crate::cpu::CPU;

define_instruction!(TSX, |cpu: &mut CPU, _, _ :u8| {
    cpu.set_x(cpu.get_s());
});
