use crate::cpu::instruction::Instruction;
use crate::cpu::opcode::OpCodeExecutor;
use crate::cpu::AddressingMode;
use crate::memory::MemoryBus;
use super::opcode::OpCode;
use super::opcode_table::OPCODE_TABLE;
use super::Status;

pub struct CPU {
    a: u8,
    x: u8,
    y: u8,
    pc: u16,             
    s: u8,
    p: u8,
    current_opcode: Option<OpCode>,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            a: 0, 
            x: 0,
            y: 0, 
            pc: 0x8000, // NES program entry point
            s: 0xFD,
            p: Status::UNUSED.bits(),
            current_opcode: None,
        }
    }

    // startregion: Functions to utilize the status register within the CPU

    pub fn is_flag_set(&self, flag: Status) -> bool {
        self.p & flag.bits() != 0
    }

    pub fn set_flag(&mut self, flag: Status, condition:bool) {
        if condition {
            self.p |= flag.bits()
        } else {
            self.p &= !flag.bits()
        }
    }

    // endregion: Functions to utilize the status register within the CPU

    pub fn get_effective_address(&self, addressing_mode: AddressingMode, memory: &MemoryBus) -> u16 {
        match addressing_mode {
            AddressingMode::Immediate => self.pc, // PC points to operand
            AddressingMode::ZeroPage => memory.read_byte(self.pc) as u16,
            AddressingMode::ZeroPageX => {
                let addr = memory.read_byte(self.pc).wrapping_add(self.x);
                addr as u16
            }
            AddressingMode::ZeroPageY => {
                let addr = memory.read_byte(self.pc).wrapping_add(self.y);
                addr as u16
            }
            AddressingMode::Absolute => memory.read_word(self.pc),
            AddressingMode::AbsoluteX => memory.read_word(self.pc).wrapping_add(self.x as u16),
            AddressingMode::AbsoluteY => memory.read_word(self.pc).wrapping_add(self.y as u16),
            AddressingMode::Indirect => {
                let addr = memory.read_word(self.pc);
                memory.read_word(addr) // Indirect fetch
            }
            AddressingMode::IndirectX => {
                let base = memory.read_byte(self.pc).wrapping_add(self.x);
                memory.read_word(base as u16) // Read pointer from zero page
            }
            AddressingMode::IndirectY => {
                let base = memory.read_byte(self.pc) as u16;
                memory.read_word(base).wrapping_add(self.y as u16)
            }
            AddressingMode::Relative => {
                let offset = memory.read_byte(self.pc) as i8 as i16; // Signed offset
                self.pc.wrapping_add(1).wrapping_add(offset as u16)
            }
            AddressingMode::Accumulator => panic!("Accumulator mode does not have an address"),
            AddressingMode::Implied => panic!("Implied mode does not have an address"),
        }
    }    

    pub fn step(&mut self, memory: &mut MemoryBus) {
        let opcode = self.fetch_byte(memory);
        self.execute(opcode, memory);
    }

    pub fn pull_stack(&mut self, memory: &MemoryBus) -> u8 {
        // Stack is located at $0100–$01FF, so we add 0x100 to the stack pointer
        let address = 0x0100 | self.get_s() as u16;

        // Read the value from memory at the computed stack address
        let value = memory.read_byte(address);

        // Increment stack pointer (stack grows downward in 6502, so popping moves it up)
        self.set_s(self.get_s().wrapping_add(1));

        value
    }
    
    pub fn push_stack(&mut self, memory: &mut MemoryBus, value: u8) {
        let stack_address = 0x0100 | (self.s as u16); // Stack is at $0100 - $01FF
        memory.write(stack_address, value);  // Store value in memory
        self.s = self.s.wrapping_sub(1); // Decrement SP
    }

    // region: Setter / Getter methods

    pub fn set_current_opcode(&mut self, opcode: OpCode) {
        self.current_opcode = Some(opcode);
    }

    pub fn get_current_opcode(&self) -> Option<&OpCode> {
        self.current_opcode.as_ref()
    }

    pub fn get_s(&self) -> u8 {
        self.s
    }

    pub fn set_s(&mut self, value: u8) {
        self.s = value;
    }

    pub fn get_pc(&self) -> u16 {
        self.pc
    }

    pub fn set_pc(&mut self, value: u16) {
        self.pc = value
    }

    pub fn get_a(&self) -> u8 {
        self.a
    }

    pub fn set_a(&mut self, value: u8) {
        self.a = value;
    }

    pub fn get_x(&self) -> u8 {
        self.x
    }

    pub fn set_x(&mut self, value: u8) {
        self.x = value;
    }

    pub fn get_y(&self) -> u8 {
        self.y
    }

    pub fn set_y(&mut self, value: u8) {
        self.y = value;
    }

    pub fn branch(&mut self, _: &mut MemoryBus, condition: bool, offset: u8) {
        if condition {
            let signed_offset = offset as i8 as i16; // Sign-extend the 8-bit offset
            let new_pc = self.get_pc().wrapping_add(signed_offset as u16);
            self.set_pc(new_pc);
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
        self.set_flag(Status::ZERO, value == 0);
        self.set_flag(Status::NEGATIVE, value & 0b1000_000 != 0);
    }

    /**
     * Executes the given op code executor.
     */
    #[inline(always)]
    pub fn execute_instruction<T: Instruction>(&mut self, executor: OpCodeExecutor<T>, memory: &mut MemoryBus) {
        executor.execute(self, memory);
    }

    pub fn reset(&mut self, memory: &MemoryBus) {
        let low_byte = memory.read_byte(MemoryBus::RESET_VECTOR_ADDR) as u16;
        let high_byte = memory.read_byte(MemoryBus::RESET_VECTOR_HIGH_ADDR) as u16;

        self.pc = (high_byte << 8) | low_byte;

        // Reset the processor status
        self.p = Status::UNUSED.bits()
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
        memory.read_byte(address)
    }

    pub fn fetch_zero_page_x(&mut self, memory: &MemoryBus) -> u8 {
        let address = self.fetch_byte(memory).wrapping_add(self.x) as u16;
        memory.read_byte(address)
    }

    pub fn fetch_zero_page_y(&mut self, memory: &MemoryBus) -> u8 {
        let address = self.fetch_byte(memory).wrapping_add(self.y) as u16;
        memory.read_byte(address)
    }

    pub fn fetch_absolute(&mut self, memory: &MemoryBus) -> u8 {
        let low = self.fetch_byte(memory) as u16;
        let high = self.fetch_byte(memory) as u16;
        let address = (high << 8) | low;
        memory.read_byte(address)
    }

    pub fn fetch_absolute_x(&mut self, memory: &MemoryBus) -> u8 {
        let low = self.fetch_byte(memory) as u16;
        let high = self.fetch_byte(memory) as u16;
        let base_address = (high << 8) | low;
        let address = base_address.wrapping_add(self.x as u16);
        memory.read_byte(address)
    }

    pub fn fetch_absolute_y(&mut self, memory: &MemoryBus) -> u8 {
        let low = self.fetch_byte(memory) as u16;
        let high = self.fetch_byte(memory) as u16;
        let base_address = (high << 8) | low;
        let address = base_address.wrapping_add(self.y as u16);
        memory.read_byte(address)
    }

    pub fn fetch_indirect(&mut self, memory: &MemoryBus) -> u8 {
        let base_address = self.fetch_word(memory); // Fetch a 16-bit address
    
        // Handle the 6502's infamous indirect jump bug
        let low = memory.read_byte(base_address) as u16;
        let high = memory.read_byte((base_address & 0xFF00) | ((base_address + 1) & 0x00FF)) as u16; 
    
        let effective_address = (high << 8) | low;
    
        memory.read_byte(effective_address)
    }    

    pub fn fetch_indirect_x(&mut self, memory: &MemoryBus) -> u8 {
        let base_address = self.fetch_byte(memory);
        let zero_page_address = base_address.wrapping_add(self.x);

        let low = memory.read_byte(zero_page_address as u16) as u16;
        let high = memory.read_byte(zero_page_address.wrapping_add(1) as u16) as u16;
        let effective_address = (high << 8) | low;

        memory.read_byte(effective_address)
    }

    pub fn fetch_indirect_y(&mut self, memory: &MemoryBus) -> u8 {
        let base_address = self.fetch_byte(memory);

        let low = memory.read_byte(base_address as u16) as u16;
        let high = memory.read_byte(base_address.wrapping_add(1) as u16) as u16;
        let base_address = (high << 8) | low;
        let effective_address = base_address.wrapping_add(self.y as u16);

        memory.read_byte(effective_address)
    }

    pub fn fetch_byte(&mut self, memory: &MemoryBus) -> u8 {
        let opcode = memory.read_byte(self.pc);
        self.pc = self.pc.wrapping_add(1);
        opcode
    }

    pub fn fetch_word(&mut self, memory: &MemoryBus) -> u16 {
        let low = memory.read_byte(self.pc) as u16;
        let high = memory.read_byte(self.pc.wrapping_add(1)) as u16;
        self.pc = self.pc.wrapping_add(2);
        (high << 8) | low // Little-endian: low byte first, then high byte
    }

    // endregion: Fetch functions.
}