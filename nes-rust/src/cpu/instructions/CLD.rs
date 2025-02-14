use crate::cpu::CPU;
use crate::instruction::Instruction;
use crate::opcode::OpCode;

pub struct CLD;

#[inline(always)]
impl Instruction for CLD {
    fn execute(&self, cpu: &mut CPU, opcode: &OpCode) {
        // TODO: Implement execution logic for CLD
        println!("Executing CLD instruction");
    }
}
