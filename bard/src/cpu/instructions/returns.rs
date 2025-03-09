use crate::cpu::CPU;
use crate::{cpu::mnemonic::Mnemonic, memory::CPUBus};

const BREAK_FLAG_MASK: u8 = 0b00010000; // Bit 4 (Break Flag)
const UNUSED_FLAG_MASK: u8 = 0b00100000; // Bit 5 (Unused Flag)
const STATUS_FLAG_MASK: u8 = !(BREAK_FLAG_MASK | UNUSED_FLAG_MASK);

impl CPU {
    pub fn handle_return(&mut self, mnemonic: &Mnemonic, memory: &mut CPUBus) {
        match mnemonic {
            Mnemonic::RTS => {
                let high = self.pull_stack(memory) as u16; 
                let low = self.pull_stack(memory) as u16;
                self.set_pc(((high << 8) | low).wrapping_add(1));
            }
            Mnemonic::RTI => {
                let new_processor_status = self.pull_stack(memory);
                self.set_s((new_processor_status & STATUS_FLAG_MASK) | UNUSED_FLAG_MASK); 

                let high = self.pull_stack(memory) as u16;
                let low = self.pull_stack(memory) as u16;
                self.set_pc((high << 8) | low);
            }
            _ => {}
        }
    }
}
