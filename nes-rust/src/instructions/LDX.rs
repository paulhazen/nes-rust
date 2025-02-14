use crate::cpu::CPU;
use crate::instruction::Instruction;
use crate::opcode::OpCode;

pub struct LDX;

#[inline(always)]
impl Instruction for LDX {
    fn execute(&self, cpu: &mut CPU, opcode: &OpCode) {
        // TODO: Implement execution logic for LDX
        println!("Executing LDX instruction");
    }
}
