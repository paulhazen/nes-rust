use crate::cpu::CPU;
use crate::instruction::Instruction;
use crate::opcode::OpCode;

pub struct TYA;

#[inline(always)]
impl Instruction for TYA {
    fn execute(&self, cpu: &mut CPU, opcode: &OpCode) {
        // TODO: Implement execution logic for TYA
        println!("Executing TYA instruction");
    }
}
