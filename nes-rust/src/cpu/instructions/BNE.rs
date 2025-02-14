use crate::cpu::CPU;
use crate::instruction::Instruction;
use crate::opcode::OpCode;

pub struct BNE;

#[inline(always)]
impl Instruction for BNE {
    fn execute(&self, cpu: &mut CPU, opcode: &OpCode) {
        // TODO: Implement execution logic for BNE
        println!("Executing BNE instruction");
    }
}
