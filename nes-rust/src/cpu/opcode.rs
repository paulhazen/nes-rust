use crate::cpu::instruction::Instruction;
use crate::cpu::CPU;
use crate::cpu::instruction_mnemonic::InstructionMnemonic;
use crate::cpu::addressing_mode::AddressingMode;
use crate::memory::MemoryBus;

pub type InstructionFactory = fn() -> Box<dyn Instruction + 'static>;

#[derive(Clone)]
pub struct OpCode {
    // The Three letter mnemonic for the particular opcode.
    pub mnemonic: InstructionMnemonic, 

    // The addressing mode for this opcode.
    pub mode: AddressingMode,

    // The size of the opcode.
    pub size: u8,

    // The number of clock cycles the opcode takes to execute.
    pub cycles: u8,

    // Function to create the instruction struct that implements the 
    // Instruction trait.
    pub factory: InstructionFactory,
}

pub struct OpCodeExecutor<T: Instruction> {
    opcode: OpCode,
    executor: T,
}

impl<T: Instruction> OpCodeExecutor<T> {
    pub fn new(opcode: OpCode, executor: T) -> Self {
        Self {opcode, executor }
    }

    pub fn execute(&self, cpu: &mut CPU, memory: &mut MemoryBus) {
        self.executor.execute(cpu, &self.opcode, memory);
    }
}