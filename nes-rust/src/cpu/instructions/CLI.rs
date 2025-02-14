use crate::cpu::CPU;
use crate::instruction::Instruction;
use crate::opcode::OpCode;

pub struct CLI;

#[inline(always)]
impl Instruction for CLI {
    fn execute(&self, cpu: &mut CPU, opcode: &OpCode) {
        // TODO: Implement execution logic for CLI
        println!("Executing CLI instruction");
    }
}
