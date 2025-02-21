use crate::cpu::status_register::StatusRegister;
use crate::cpu::instruction::Instruction;
use crate::cpu::opcode::OpCodeExecutor;
use crate::memory::MemoryBus;
use super::opcode::OpCode;
use super::opcode_table::OPCODE_TABLE;

pub struct CPU {
    accumulator: u8,
    x_register: u8,
    y_register: u8,
    program_counter: u16,             
    stack_pointer: u8,
    pub processor_status: StatusRegister,
    current_opcode: Option<OpCode>,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            accumulator: 0, 
            x_register: 0,
            y_register: 0, 
            program_counter: 0x8000, // NES program entry point
            stack_pointer: 0xFD,
            processor_status: StatusRegister::new(),
            current_opcode: None,
        }
    }

    pub fn step(&mut self, memory: &mut MemoryBus) {
        let opcode = self.fetch_byte(memory);
        self.execute(opcode, memory);
    }

    pub fn pull_stack(&mut self, memory: &MemoryBus) -> u8 {
        // Stack is located at $0100–$01FF, so we add 0x100 to the stack pointer
        let address = 0x0100 | self.get_stack_pointer() as u16;

        // Read the value from memory at the computed stack address
        let value = memory.read(address);

        // Increment stack pointer (stack grows downward in 6502, so popping moves it up)
        self.set_stack_pointer(self.get_stack_pointer().wrapping_add(1));

        value
    }
    
    pub fn push_stack(&mut self, memory: &mut MemoryBus, value: u8) {
        let stack_address = 0x0100 | (self.stack_pointer as u16); // Stack is at $0100 - $01FF
        memory.write(stack_address, value);  // Store value in memory
        self.stack_pointer = self.stack_pointer.wrapping_sub(1); // Decrement SP
    }

    // region: Setter / Getter methods

    pub fn set_current_opcode(&mut self, opcode: OpCode) {
        self.current_opcode = Some(opcode);
    }

    pub fn get_current_opcode(&self) -> Option<&OpCode> {
        self.current_opcode.as_ref()
    }

    pub fn get_stack_pointer(&self) -> u8 {
        self.stack_pointer
    }

    pub fn set_stack_pointer(&mut self, value: u8) {
        self.stack_pointer = value;
    }

    pub fn get_program_counter(&self) -> u16 {
        self.program_counter
    }

    pub fn set_program_counter(&mut self, value: u16) {
        self.program_counter = value
    }

    pub fn get_accumulator(&self) -> u8 {
        self.accumulator
    }

    pub fn set_accumulator(&mut self, value: u8) {
        self.accumulator = value;
    }

    pub fn get_x_register(&self) -> u8 {
        self.x_register
    }

    pub fn set_x_register(&mut self, value: u8) {
        self.x_register = value;
    }

    pub fn get_y_register(&self) -> u8 {
        self.y_register
    }

    pub fn set_y_register(&mut self, value: u8) {
        self.y_register = value;
    }

    pub fn branch(&mut self, _: &mut MemoryBus, condition: bool, offset: u8) {
        if condition {
            let signed_offset = offset as i8 as i16; // Sign-extend the 8-bit offset
            let new_pc = self.get_program_counter().wrapping_add(signed_offset as u16);
            self.set_program_counter(new_pc);
        }
    }
    
    // endregion: Accessor methods

    pub fn execute(&mut self, opcode: u8, memory: &mut MemoryBus) {
        if let Some(instruction) = OPCODE_TABLE.get(&opcode) {
            
            self.set_current_opcode(instruction.clone());

            let test = (instruction.factory)();

            println!("{0:#?} {1:#x}", instruction.mnemonic, opcode);

            test.execute(self, instruction, memory);
        } else {
            //println!("Could not find instruction for opcode \"{:#x}\".", opcode);
        }
    }

    pub fn dbg_view_opcode_table(&self) {
        println!("=== START OPCODE_TABLE ===");
        for (key, value) in OPCODE_TABLE.iter() {
            println!("{0:>3}   {1:#?}   ({3} bytes {4} cycles); Mode: {2:#?}", key, value.mnemonic, value.mode, value.size, value.cycles);
        }        
        println!("===  END OPCODE_TABLE  ===")
    }

    pub fn update_zero_and_negative_flags(&mut self, value: u8) {
        // Update Zero flag (Z): Set if value is 0
        if value == 0 {
            self.processor_status.set(StatusRegister::ZERO);
        }

        // Update Negative flag (N): Set if the highest bit (bit 7) is 1
        if value & 0b1000_0000 != 0 {
            self.processor_status.set(StatusRegister::NEGATIVE);
        }
    }

    /**
     * Executes the given op code executor.
     */
    #[inline(always)]
    pub fn execute_instruction<T: Instruction>(&mut self, executor: OpCodeExecutor<T>, memory: &mut MemoryBus) {
        executor.execute(self, memory);
    }

    pub fn reset(&mut self, memory: &MemoryBus) {
        let low_byte = memory.read(MemoryBus::RESET_VECTOR_ADDR) as u16;
        let high_byte = memory.read(MemoryBus::RESET_VECTOR_HIGH_ADDR) as u16;

        self.program_counter = (high_byte << 8) | low_byte;

        // Reset the processor status
        self.processor_status.reset()
    }

    // startregion: Fetch functions

    pub fn fetch_relative(&mut self, memory: &mut MemoryBus) -> u8 {
        return self.fetch_byte(memory);
    }

    pub fn fetch_immediate(&mut self, memory: &MemoryBus) -> u8 {
        self.fetch_byte(memory)
    }

    pub fn fetch_zero_page(&mut self, memory: &MemoryBus) -> u8 {
        let address = self.fetch_byte(memory) as u16;
        memory.read(address)
    }

    pub fn fetch_zero_page_x(&mut self, memory: &MemoryBus) -> u8 {
        let address = self.fetch_byte(memory).wrapping_add(self.get_x_register()) as u16;
        memory.read(address)
    }

    pub fn fetch_zero_page_y(&mut self, memory: &MemoryBus) -> u8 {
        let address = self.fetch_byte(memory).wrapping_add(self.y_register) as u16;
        memory.read(address)
    }

    pub fn fetch_absolute(&mut self, memory: &MemoryBus) -> u8 {
        let low = self.fetch_byte(memory) as u16;
        let high = self.fetch_byte(memory) as u16;
        let address = (high << 8) | low;
        memory.read(address)
    }

    pub fn fetch_absolute_x(&mut self, memory: &MemoryBus) -> u8 {
        let low = self.fetch_byte(memory) as u16;
        let high = self.fetch_byte(memory) as u16;
        let base_address = (high << 8) | low;
        let address = base_address.wrapping_add(self.get_x_register() as u16);
        memory.read(address)
    }

    pub fn fetch_absolute_y(&mut self, memory: &MemoryBus) -> u8 {
        let low = self.fetch_byte(memory) as u16;
        let high = self.fetch_byte(memory) as u16;
        let base_address = (high << 8) | low;
        let address = base_address.wrapping_add(self.get_y_register() as u16);
        memory.read(address)
    }

    pub fn fetch_indirect(&mut self, memory: &MemoryBus) -> u8 {
        let base_address = self.fetch_word(memory); // Fetch a 16-bit address
    
        // Handle the 6502's infamous indirect jump bug
        let low = memory.read(base_address) as u16;
        let high = memory.read((base_address & 0xFF00) | ((base_address + 1) & 0x00FF)) as u16; 
    
        let effective_address = (high << 8) | low;
    
        memory.read(effective_address)
    }    

    pub fn fetch_indirect_x(&mut self, memory: &MemoryBus) -> u8 {
        let base_address = self.fetch_byte(memory);
        let zero_page_address = base_address.wrapping_add(self.get_x_register());

        let low = memory.read(zero_page_address as u16) as u16;
        let high = memory.read(zero_page_address.wrapping_add(1) as u16) as u16;
        let effective_address = (high << 8) | low;

        memory.read(effective_address)
    }

    pub fn fetch_indirect_y(&mut self, memory: &MemoryBus) -> u8 {
        let base_address = self.fetch_byte(memory);

        let low = memory.read(base_address as u16) as u16;
        let high = memory.read(base_address.wrapping_add(1) as u16) as u16;
        let base_address = (high << 8) | low;
        let effective_address = base_address.wrapping_add(self.get_y_register() as u16);

        memory.read(effective_address)
    }

    pub fn fetch_byte(&mut self, memory: &MemoryBus) -> u8 {
        let opcode = memory.read(self.program_counter);
        self.program_counter = self.program_counter.wrapping_add(1);
        opcode
    }

    pub fn fetch_word(&mut self, memory: &MemoryBus) -> u16 {
        let low = memory.read(self.program_counter) as u16;
        let high = memory.read(self.program_counter.wrapping_add(1)) as u16;
        self.program_counter = self.program_counter.wrapping_add(2);
        (high << 8) | low // Little-endian: low byte first, then high byte
    }

    // endregion: Fetch functions.
}