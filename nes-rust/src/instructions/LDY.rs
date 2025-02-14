use crate::cpu::CPU;
use crate::instruction::Instruction;
use crate::opcode::OpCode;

pub struct LDY;

#[inline(always)]
impl Instruction for LDY {
    fn execute(&self, cpu: &mut CPU, opcode: &OpCode) {
        // TODO: Implement execution logic for LDY
        println!("Executing LDY instruction");
    }
}
