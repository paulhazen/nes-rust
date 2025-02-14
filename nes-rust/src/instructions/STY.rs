use crate::cpu::CPU;
use crate::instruction::Instruction;
use crate::opcode::OpCode;

pub struct STY;

#[inline(always)]
impl Instruction for STY {
    fn execute(&self, cpu: &mut CPU, opcode: &OpCode) {
        // TODO: Implement execution logic for STY
        println!("Executing STY instruction");
    }
}
