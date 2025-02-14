use crate::cpu::CPU;
use crate::instruction::Instruction;
use crate::opcode::OpCode;

pub struct BCC;

#[inline(always)]
impl Instruction for BCC {
    fn execute(&self, cpu: &mut CPU, opcode: &OpCode) {
        // TODO: Implement execution logic for BCC
        println!("Executing BCC instruction");
    }
}
