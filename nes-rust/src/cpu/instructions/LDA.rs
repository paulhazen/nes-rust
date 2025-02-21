use crate::cpu::CPU;
use crate::cpu::instruction::Instruction;
use crate::cpu::opcode::OpCode;
use crate::define_instruction;

define_instruction!(LDA, |cpu: &mut CPU, _, value| {
    cpu.set_accumulator(value);
    cpu.update_zero_and_negative_flags(value);
}); 
