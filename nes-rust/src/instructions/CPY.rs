use crate::cpu::CPU;
use crate::instruction::Instruction;
use crate::opcode::OpCode;

pub struct CPY;

#[inline(always)]
impl Instruction for CPY {
    fn execute(&self, cpu: &mut CPU, opcode: &OpCode) {
        // TODO: Implement execution logic for CPY
        println!("Executing CPY instruction");
    }
}
