use std::fmt;
use crate::memory::CPUBus;
use crate::memory::Bus;
use super::instruction_metadata::InstructionMetadata;
use super::opcode_table::OPCODE_TABLE;
use super::Status;

pub struct CPU {
    // Accumulator
    a: u8,

    // X register
    x: u8,

    // Y register
    y: u8,

    // Program counter register
    pc: u16,          

    // Stack register   
    s: u8,

    // Status register
    p: u8,

    logging: bool,

    current_instruction: Option<InstructionMetadata>,
}

impl fmt::Display for CPU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let status_chars = [
            if (self.p & 0b1000_0000) != 0 { 'N' } else { 'n' }, // Negative
            if (self.p & 0b0100_0000) != 0 { 'V' } else { 'v' }, // Overflow
            if (self.p & 0b0010_0000) != 0 { 'U' } else { 'u' }, // Unused (always 1)
            if (self.p & 0b0001_0000) != 0 { 'B' } else { 'b' }, // Break
            if (self.p & 0b0000_1000) != 0 { 'D' } else { 'd' }, // Decimal (ignored on NES)
            if (self.p & 0b0000_0100) != 0 { 'I' } else { 'i' }, // Interrupt Disable
            if (self.p & 0b0000_0010) != 0 { 'Z' } else { 'z' }, // Zero
            if (self.p & 0b0000_0001) != 0 { 'C' } else { 'c' }, // Carry
        ];

        write!(
            f,
            "A:{:02X} X:{:02X} Y:{:02X} S:{:02X} P:{}",
            self.a,
            self.x,
            self.y,
            self.s,
            status_chars.iter().collect::<String>()
        )
    }
}

impl CPU {
    pub const SIGN_BIT: u8 = 0x80;
    
    pub fn new() -> Self {
        CPU {
            a: 0, 
            x: 0,
            y: 0, 
            pc: 0x8000, // NES program entry point
            s: 0xFD,
            p: Status::UNUSED.bits() | Status::INTERRUPT_DISABLE.bits(),
            logging: true,
            current_instruction: None,
        }
    }

    // region: Functions to utilize the status register within the CPU

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

    pub fn step(&mut self, memory: &mut CPUBus) -> u8{
        let opcode = self.fetch_byte(memory);

        if self.logging {
            let disassembly = self.disassemble_instruction(self.pc, memory);
            println!("{}", disassembly);
        }
 
        self.execute_instruction(&opcode, memory)
    }

    pub fn pull_stack(&mut self, memory: &CPUBus) -> u8 {
        // Stack is located at $0100–$01FF, so we add 0x100 to the stack pointer
        let address = 0x0100 | self.get_s() as u16;

        // Read the value from memory at the computed stack address
        let value = memory.read_byte(address);

        // Increment stack pointer (stack grows downward in 6502, so popping moves it up)
        self.set_s(self.get_s().wrapping_add(1));

        value
    }
    
    pub fn push_stack(&mut self, memory: &mut CPUBus, value: u8) {
        let stack_address = 0x0100 | (self.s as u16); // Stack is at $0100 - $01FF
        memory.default_write_byte(stack_address, value);  // Store value in memory
        self.s = self.s.wrapping_sub(1); // Decrement SP
    }

    // region: Setter / Getter methods

    pub fn set_current_opcode(&mut self, opcode: InstructionMetadata) {
        self.current_instruction = Some(opcode);
    }

    pub fn get_current_opcode(&self) -> Option<&InstructionMetadata> {
        self.current_instruction.as_ref()
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

    pub fn branch(&mut self, _: &mut CPUBus, condition: bool, offset: u8) {
        if condition {
            let signed_offset = offset as i8 as i16; // Sign-extend the 8-bit offset
            let new_pc = self.get_pc().wrapping_add(signed_offset as u16);
            self.set_pc(new_pc);
        }
    }

    // endregion: Accessor methods

    pub fn dbg_view_opcode_table(&self) {
        println!("=== START OPCODE_TABLE ===");
        for (key, value) in OPCODE_TABLE.iter() {
            println!("{0:>3}   {1:#?}   ({3} bytes {4} cycles); Mode: {2:#?}", key, value.mnemonic, value.addressing_mode, value.size, value.cycle_count);
        }        
        println!("===  END OPCODE_TABLE  ===")
    }

    pub fn update_zero_and_negative_flags(&mut self, value: u8) {
        self.set_flag(Status::ZERO, value == 0);
        self.set_flag(Status::NEGATIVE, value & 0b1000_000 != 0);
    }

    pub fn reset(&mut self, memory: &CPUBus) {
        self.pc = memory.read_word(0xFFFC);

        self.a = 0x00;
        self.x = 0x00;
        self.y = 0x00;
        self.s = 0xFD;
        self.p = 0x43;
    }
    

    // startregion: Fetch functions

    pub fn fetch_relative(&mut self, memory: &mut CPUBus) -> u8 {
        return self.fetch_byte(memory);
    }

    pub fn fetch_immediate(&mut self, memory: &CPUBus) -> u8 {
        self.fetch_byte(memory)
    }

    pub fn fetch_zero_page(&mut self, memory: &CPUBus) -> u8 {
        let address = self.fetch_byte(memory) as u16;
        memory.read_byte(address)
    }

    pub fn fetch_zero_page_x(&mut self, memory: &CPUBus) -> u8 {
        let address = self.fetch_byte(memory).wrapping_add(self.x) as u16;
        memory.read_byte(address)
    }

    pub fn fetch_zero_page_y(&mut self, memory: &CPUBus) -> u8 {
        let address = self.fetch_byte(memory).wrapping_add(self.y) as u16;
        memory.read_byte(address)
    }

    pub fn fetch_absolute(&mut self, memory: &CPUBus) -> u8 {
        let low = self.fetch_byte(memory) as u16;
        let high = self.fetch_byte(memory) as u16;
        let address = (high << 8) | low;
        memory.read_byte(address)
    }

    pub fn fetch_absolute_x(&mut self, memory: &CPUBus) -> u8 {
        let low = self.fetch_byte(memory) as u16;
        let high = self.fetch_byte(memory) as u16;
        let base_address = (high << 8) | low;
        let address = base_address.wrapping_add(self.x as u16);
        memory.read_byte(address)
    }

    pub fn fetch_absolute_y(&mut self, memory: &CPUBus) -> u8 {
        let low = self.fetch_byte(memory) as u16;
        let high = self.fetch_byte(memory) as u16;
        let base_address = (high << 8) | low;
        let address = base_address.wrapping_add(self.y as u16);
        memory.read_byte(address)
    }

    pub fn fetch_indirect(&mut self, memory: &CPUBus) -> u8 {
        let base_address = self.fetch_word(memory); // Fetch a 16-bit address
    
        // Handle the 6502's infamous indirect jump bug
        let low = memory.read_byte(base_address) as u16;
        let high = memory.read_byte((base_address & 0xFF00) | ((base_address + 1) & 0x00FF)) as u16; 
    
        let effective_address = (high << 8) | low;
    
        memory.read_byte(effective_address)
    }    

    pub fn fetch_indirect_x(&mut self, memory: &CPUBus) -> u8 {
        let base_address = self.fetch_byte(memory);
        let zero_page_address = base_address.wrapping_add(self.x);

        let low = memory.read_byte(zero_page_address as u16) as u16;
        let high = memory.read_byte(zero_page_address.wrapping_add(1) as u16) as u16;
        let effective_address = (high << 8) | low;

        memory.read_byte(effective_address)
    }

    pub fn fetch_indirect_y(&mut self, memory: &CPUBus) -> u8 {
        let base_address = self.fetch_byte(memory);

        let low = memory.read_byte(base_address as u16) as u16;
        let high = memory.read_byte(base_address.wrapping_add(1) as u16) as u16;
        let base_address = (high << 8) | low;
        let effective_address = base_address.wrapping_add(self.y as u16);

        memory.read_byte(effective_address)
    }

    pub fn fetch_byte(&mut self, memory: &CPUBus) -> u8 {
        let opcode = memory.read_byte(self.pc);
        self.pc = self.pc.wrapping_add(1);
        opcode
    }

    pub fn fetch_word(&mut self, memory: &CPUBus) -> u16 {
        let low = memory.read_byte(self.pc) as u16;
        let high = memory.read_byte(self.pc.wrapping_add(1)) as u16;
        self.pc = self.pc.wrapping_add(2);
        (high << 8) | low // Little-endian: low byte first, then high byte
    }

    // endregion: Fetch functions.
}