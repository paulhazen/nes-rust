use crate::cpu::CPU;
use crate::instruction::Instruction;
use crate::opcode::OpCode;

pub struct BVC;

#[inline(always)]
impl Instruction for BVC {
    fn execute(&self, cpu: &mut CPU, opcode: &OpCode) {
        // TODO: Implement execution logic for BVC
        println!("Executing BVC instruction");
    }
}
