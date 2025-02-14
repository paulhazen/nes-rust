use crate::cpu::CPU;
use crate::instruction::Instruction;
use crate::opcode::OpCode;

pub struct LDA;

#[inline(always)]
impl Instruction for LDA {
    fn execute(&self, cpu: &mut CPU, opcode: &OpCode) {
        // TODO: Implement execution logic for LDA
        println!("Executing LDA instruction");
    }
}
