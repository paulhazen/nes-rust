use crate::cpu::CPU;
use crate::instruction::Instruction;
use crate::opcode::OpCode;

pub struct SED;

#[inline(always)]
impl Instruction for SED {
    fn execute(&self, cpu: &mut CPU, opcode: &OpCode) {
        // TODO: Implement execution logic for SED
        println!("Executing SED instruction");
    }
}
