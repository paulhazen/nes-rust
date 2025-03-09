// STA, STX, STY

use crate::cpu::mnemonic::Mnemonic;
use crate::cpu::CPU;
use crate::memory::CPUBus;
use crate::memory::Bus;

impl CPU {
    pub fn handle_store(&mut self, operand: &u8, mnemonic: &Mnemonic, memory: &mut CPUBus){

        // This *might* be incorrect - the operand should be an address determined by the
        // addressing mode. I think this is correct though.
        let address = *operand as u16;

        let value = match mnemonic {
            Mnemonic::STA => self.get_a(),
            Mnemonic::STX => self.get_x(),
            Mnemonic::STY => self.get_y(),
            _ => return,
        };

        memory.write_byte(address, value);
    }   
}