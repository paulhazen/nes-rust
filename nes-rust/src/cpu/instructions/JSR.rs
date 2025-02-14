use crate::cpu::CPU;
use crate::instruction::Instruction;
use crate::opcode::OpCode;

pub struct JSR;

#[inline(always)]
impl Instruction for JSR {
    fn execute(&self, cpu: &mut CPU, opcode: &OpCode) {
        // TODO: Implement execution logic for JSR
        println!("Executing JSR instruction");
    }
}
