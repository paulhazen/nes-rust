use crate::cpu::CPU;
use crate::instruction::Instruction;
use crate::opcode::OpCode;

pub struct TSX;

#[inline(always)]
impl Instruction for TSX {
    fn execute(&self, cpu: &mut CPU, opcode: &OpCode) {
        // TODO: Implement execution logic for TSX
        println!("Executing TSX instruction");
    }
}
