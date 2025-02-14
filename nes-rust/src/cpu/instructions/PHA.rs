use crate::cpu::CPU;
use crate::instruction::Instruction;
use crate::opcode::OpCode;

pub struct PHA;

#[inline(always)]
impl Instruction for PHA {
    fn execute(&self, cpu: &mut CPU, opcode: &OpCode) {
        // TODO: Implement execution logic for PHA
        println!("Executing PHA instruction");
    }
}
