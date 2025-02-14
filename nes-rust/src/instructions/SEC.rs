use crate::cpu::CPU;
use crate::instruction::Instruction;
use crate::opcode::OpCode;

pub struct SEC;

#[inline(always)]
impl Instruction for SEC {
    fn execute(&self, cpu: &mut CPU, opcode: &OpCode) {
        // TODO: Implement execution logic for SEC
        println!("Executing SEC instruction");
    }
}
