use crate::cpu::CPU;
use crate::instruction::Instruction;
use crate::opcode::OpCode;

pub struct STA;

#[inline(always)]
impl Instruction for STA {
    fn execute(&self, cpu: &mut CPU, opcode: &OpCode) {
        // TODO: Implement execution logic for STA
        println!("Executing STA instruction");
    }
}
