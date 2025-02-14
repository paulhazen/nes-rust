use crate::cpu::CPU;
use crate::instruction::Instruction;
use crate::opcode::OpCode;

pub struct CMP;

#[inline(always)]
impl Instruction for CMP {
    fn execute(&self, cpu: &mut CPU, opcode: &OpCode) {
        // TODO: Implement execution logic for CMP
        println!("Executing CMP instruction");
    }
}
