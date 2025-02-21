use crate::cpu::CPU;
use crate::cpu::instruction::Instruction;
use crate::cpu::opcode::OpCode;
use crate::define_instruction;

define_instruction!(LDY, |cpu: &mut CPU, _, value| {
    cpu.set_y_register(value);
});
