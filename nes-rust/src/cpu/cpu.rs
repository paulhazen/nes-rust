use crate::cpu::status_register::StatusRegister;
use crate::cpu::instruction::Instruction;
use crate::cpu::opcode::OpCodeExecutor;
use crate::memory::MemoryBus;

pub struct CPU<'a> {
    accumulator: u8,
    x_register: u8,
    y_register: u8,
    program_counter: u16,             
    stack_pointer: u8,
    pub processor_status: StatusRegister,
    memory_bus: &'a MemoryBus,
}

impl<'a> CPU<'a> {
    pub fn new(memory_bus: &'a MemoryBus) -> Self {
        CPU {
            accumulator: 0, 
            x_register: 0,
            y_register: 0, 
            program_counter: 0x8000, // NES program entry point
            stack_pointer: 0xFD,
            processor_status: StatusRegister::new(),
            memory_bus,
        }
    }

    // region: Accessor methods

    pub fn get_program_counter(&self) -> u16 {
        self.program_counter
    }

    // endregion: Accessor methods

    /**
     * Executes the given op code executor.
     */
    #[inline(always)]
    pub fn execute_instruction<T: Instruction>(&mut self, executor: OpCodeExecutor<T>) {
        executor.execute(self);
    }

    pub fn reset(&mut self) {
        let low_byte = self.memory_bus.read(MemoryBus::RESET_VECTOR_ADDR) as u16;
        let high_byte = self.memory_bus.read(MemoryBus::RESET_VECTOR_HIGH_ADDR) as u16;

        self.program_counter = (high_byte << 8) | low_byte;

        // Reset the processor status
        self.processor_status.reset()
    }

    pub fn fetch_opcode(&mut self) -> u8 {
        let opcode = self.memory_bus.read(self.program_counter);
        self.program_counter = self.program_counter.wrapping_add(1);
        opcode
    }

    pub fn read_memory(&self, address:u16) -> u8 {
        self.memory_bus.read(address)
    }
}