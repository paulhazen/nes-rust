use crate::cpu::CPU;
use crate::instruction::Instruction;
use crate::opcode::OpCode;

pub struct AND;

#[inline(always)]
impl Instruction for AND {
    fn execute(&self, cpu: &mut CPU, opcode: &OpCode) {
        // TODO: Implement execution logic for AND
        println!("Executing AND instruction");
    }
}
