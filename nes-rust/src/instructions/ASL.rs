use crate::cpu::CPU;
use crate::instruction::Instruction;
use crate::opcode::OpCode;

pub struct ASL;

#[inline(always)]
impl Instruction for ASL {
    fn execute(&self, cpu: &mut CPU, opcode: &OpCode) {
        // TODO: Implement execution logic for ASL
        println!("Executing ASL instruction");
    }
}
