use crate::cpu::CPU;
use crate::cpu::opcode::OpCode;

pub trait Instruction {
    fn execute(&self, cpu: &mut CPU, opcode: &OpCode);
}