use crate::cpu::CPU;
use crate::instruction::Instruction;
use crate::opcode::OpCode;

pub struct NOP;

#[inline(always)]
impl Instruction for NOP {
    fn execute(&self, cpu: &mut CPU, opcode: &OpCode) {
        // TODO: Implement execution logic for NOP
        println!("Executing NOP instruction");
    }
}
