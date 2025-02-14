use crate::cpu::CPU;
use crate::instruction::Instruction;
use crate::opcode::OpCode;

pub struct BCS;

#[inline(always)]
impl Instruction for BCS {
    fn execute(&self, cpu: &mut CPU, opcode: &OpCode) {
        // TODO: Implement execution logic for BCS
        println!("Executing BCS instruction");
    }
}
