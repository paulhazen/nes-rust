use crate::{cpu::{addressing_mode::AddressingMode, opcode_table::OPCODE_TABLE, CPU}, memory::{Bus, CPUBus}};

impl CPU {
    pub fn disassemble_instruction(&self, pc: u16, memory: &CPUBus) -> String {

        // Read the opcode
        let opcode = memory.read_byte(pc);

        // Get the metadata for the instruction
        let instruction_metadata = OPCODE_TABLE.get(&opcode);

        // If the metadata could not be retrieved, stop here
        if instruction_metadata.is_none() {
            return format!("Could not get instruction metadata for ${:04X}", pc)
        }

        let mut operand_str = String::new();
        let mut mem_preview = String::new();
        let mut effective_address: Option<u16> = None;
    
        match instruction_metadata.unwrap().addressing_mode {
            AddressingMode::Immediate => {
                operand_str = format!("#${:02X}", memory.read_byte(pc + 1));
            }
            AddressingMode::ZeroPage => {
                let addr = memory.read_byte(pc + 1) as u16;
                operand_str = format!("${:02X}", addr);
                effective_address = Some(addr);
            }
            AddressingMode::ZeroPageX => {
                let addr = memory.read_byte(pc + 1).wrapping_add(self.get_x()) as u16;
                operand_str = format!("${:02X},X", addr);
                effective_address = Some(addr);
            }
            AddressingMode::ZeroPageY => {
                let addr = memory.read_byte(pc + 1).wrapping_add(self.get_y()) as u16;
                operand_str = format!("${:02X},Y", addr);
                effective_address = Some(addr);
            }
            AddressingMode::Absolute => {
                let addr = memory.read_word(pc + 1);
                operand_str = format!("${:04X}", addr);
                effective_address = Some(addr);
            }
            AddressingMode::AbsoluteX => {
                let base_addr = memory.read_word(pc + 1);
                let addr = base_addr.wrapping_add(self.get_x() as u16);
                operand_str = format!("${:04X},X", base_addr);
                effective_address = Some(addr);
            }
            AddressingMode::AbsoluteY => {
                let base_addr = memory.read_word(pc + 1);
                let addr = base_addr.wrapping_add(self.get_y() as u16);
                operand_str = format!("${:04X},Y", base_addr);
                effective_address = Some(addr);
            }
            AddressingMode::Indirect => {
                let ptr = memory.read_word(pc + 1);
                let addr = memory.read_word(ptr);
                operand_str = format!("(${:04X})", ptr);
                effective_address = Some(addr);
            }
            AddressingMode::IndirectX => { // AKA IndirectX
                let base_addr = memory.read_byte(pc + 1).wrapping_add(self.get_x()) as u16;
                let addr = memory.read_word(base_addr);
                operand_str = format!("(${:02X},X)", base_addr);
                effective_address = Some(addr);
            }
            AddressingMode::IndirectY => { // AKA IndirectY
                let base_addr = memory.read_byte(pc + 1) as u16;
                let addr = memory.read_word(base_addr).wrapping_add(self.get_y() as u16);
                operand_str = format!("(${:02X}),Y", base_addr);
                effective_address = Some(addr);
            }
            AddressingMode::Relative => {
                let offset = memory.read_byte(pc + 1) as i8;
                let target = pc.wrapping_add(2).wrapping_add(offset as u16);
                operand_str = format!("${:04X}", target);
            }
            AddressingMode::Implied => {} // No operands
            AddressingMode::Accumulator => {
                operand_str = "A".to_string();
            }
        }
    
        // Fetch memory preview for loads, stores, and read-modify-write operations
        if let Some(addr) = effective_address {
            let value_at_addr = memory.read_byte(addr);
            mem_preview = format!(" @ ${:04X} = #${:02X}", addr, value_at_addr);
        }
    
        
        // Get raw instruction bytes
        let mut opcode_bytes = format!("{:02X}", opcode);
        for i in 1..=instruction_metadata.unwrap().size {
            opcode_bytes.push_str(&format!(" {:02X}", memory.read_byte(pc + i as u16)));
        }
    
        format!(
            "${:04X}:{}  {} {}{}",
            pc, opcode_bytes, instruction_metadata.unwrap().mnemonic, operand_str, mem_preview
        )
    }    
}