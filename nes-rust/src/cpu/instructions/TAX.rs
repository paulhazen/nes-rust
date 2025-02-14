use crate::cpu::CPU;
use crate::instruction::Instruction;
use crate::opcode::OpCode;

pub struct TAX;

#[inline(always)]
impl Instruction for TAX {
    fn execute(&self, cpu: &mut CPU, opcode: &OpCode) {
        // TODO: Implement execution logic for TAX
        println!("Executing TAX instruction");
    }
}
