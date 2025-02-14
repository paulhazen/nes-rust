use crate::cpu::CPU;
use crate::instruction::Instruction;
use crate::opcode::OpCode;

pub struct ORA;

#[inline(always)]
impl Instruction for ORA {
    fn execute(&self, cpu: &mut CPU, opcode: &OpCode) {
        // TODO: Implement execution logic for ORA
        println!("Executing ORA instruction");
    }
}
