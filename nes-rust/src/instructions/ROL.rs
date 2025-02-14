use crate::cpu::CPU;
use crate::instruction::Instruction;
use crate::opcode::OpCode;

pub struct ROL;

#[inline(always)]
impl Instruction for ROL {
    fn execute(&self, cpu: &mut CPU, opcode: &OpCode) {
        // TODO: Implement execution logic for ROL
        println!("Executing ROL instruction");
    }
}
