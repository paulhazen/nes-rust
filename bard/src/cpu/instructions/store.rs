// STA, STX, STY

use crate::cpu::instruction_mnemonic::InstructionMnemonic;
use crate::cpu::CPU;
use crate::memory::CPUBus;
use crate::memory::Bus;

impl CPU {
    pub fn handle_store(&mut self, operand: &u8, mnemonic: &InstructionMnemonic, memory: &mut CPUBus){

        // This *might* be incorrect - the operand should be an address determined by the
        // addressing mode. I think this is correct though.
        let address = *operand as u16;

        let value = match mnemonic {
            InstructionMnemonic::STA => self.get_a(),
            InstructionMnemonic::STX => self.get_x(),
            InstructionMnemonic::STY => self.get_y(),
            _ => return,
        };

        memory.write_byte(address, value);
    }   
}