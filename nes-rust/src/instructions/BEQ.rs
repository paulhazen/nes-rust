use crate::cpu::CPU;
use crate::instruction::Instruction;
use crate::opcode::OpCode;

pub struct BEQ;

#[inline(always)]
impl Instruction for BEQ {
    fn execute(&self, cpu: &mut CPU, opcode: &OpCode) {
        // TODO: Implement execution logic for BEQ
        println!("Executing BEQ instruction");
    }
}
