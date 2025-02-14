use crate::cpu::CPU;
use crate::instruction::Instruction;
use crate::opcode::OpCode;

pub struct BMI;

#[inline(always)]
impl Instruction for BMI {
    fn execute(&self, cpu: &mut CPU, opcode: &OpCode) {
        // TODO: Implement execution logic for BMI
        println!("Executing BMI instruction");
    }
}
