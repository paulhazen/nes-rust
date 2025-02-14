use crate::cpu::CPU;
use crate::instruction::Instruction;
use crate::opcode::OpCode;

pub struct DEC;

#[inline(always)]
impl Instruction for DEC {
    fn execute(&self, cpu: &mut CPU, opcode: &OpCode) {
        // TODO: Implement execution logic for DEC
        println!("Executing DEC instruction");
    }
}
