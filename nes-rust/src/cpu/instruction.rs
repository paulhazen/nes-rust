use crate::cpu::CPU;
use crate::memory::CPUBus;

use super::instruction_metadata::InstructionMetadata;

pub trait Instruction {
    fn execute(&self, cpu: &mut CPU, opcode: &InstructionMetadata, memory: &mut CPUBus) -> u8;
}