use crate::cpu::instruction::Instruction;
use crate::cpu::CPU;
use crate::memory::CPUBus;

use super::instruction_metadata::InstructionMetadata;

pub struct InstructionExecutor<T: Instruction> {
    instruction_metadata: InstructionMetadata,
    executor: T,
}

impl<T: Instruction> InstructionExecutor<T> {

    pub fn new(opcode: InstructionMetadata, executor: T) -> Self {
        Self {instruction_metadata: opcode, executor }
    }

    pub fn execute(&self, cpu: &mut CPU, memory: &mut CPUBus) {
        self.executor.execute(cpu, &self.instruction_metadata, memory);
    }
}