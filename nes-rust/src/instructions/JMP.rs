use crate::cpu::CPU;
use crate::instruction::Instruction;
use crate::opcode::OpCode;

pub struct JMP;

#[inline(always)]
impl Instruction for JMP {
    fn execute(&self, cpu: &mut CPU, opcode: &OpCode) {
        // TODO: Implement execution logic for JMP
        println!("Executing JMP instruction");
    }
}
