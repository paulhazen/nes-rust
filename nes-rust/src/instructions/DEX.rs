use crate::cpu::CPU;
use crate::instruction::Instruction;
use crate::opcode::OpCode;

pub struct DEX;

#[inline(always)]
impl Instruction for DEX {
    fn execute(&self, cpu: &mut CPU, opcode: &OpCode) {
        // TODO: Implement execution logic for DEX
        println!("Executing DEX instruction");
    }
}
