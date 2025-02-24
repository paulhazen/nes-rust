use crate::cpu::CPU;
use crate::memory::MemoryBus;

use super::instruction_metadata::InstructionMetadata;

pub trait Instruction {
    fn execute(&self, cpu: &mut CPU, opcode: &InstructionMetadata, memory: &mut MemoryBus) -> u8;
}