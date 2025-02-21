use crate::cpu::CPU;
use crate::cpu::opcode::OpCode;
use crate::memory::MemoryBus;

pub trait Instruction {
    fn execute(&self, cpu: &mut CPU, opcode: &OpCode, memory: &mut MemoryBus);
}