use crate::cpu::CPU;
use crate::instruction::Instruction;
use crate::opcode::OpCode;

pub struct INX;

#[inline(always)]
impl Instruction for INX {
    fn execute(&self, cpu: &mut CPU, opcode: &OpCode) {
        // TODO: Implement execution logic for INX
        println!("Executing INX instruction");
    }
}
