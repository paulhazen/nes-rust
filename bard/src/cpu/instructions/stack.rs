// PHA, PHP, PLA, PLP, TSX, TXS
use crate::cpu::mnemonic::Mnemonic;
use crate::cpu::CPU;
use crate::memory::CPUBus;

impl CPU {
    pub fn handle_stack(&mut self, mnemonic: &Mnemonic, memory: &mut CPUBus) {
        match mnemonic {
            Mnemonic::PHA => self.push_stack(memory, self.get_a()),
            Mnemonic::PHP => {
                let processor_status = self.get_s() | 0b00100000; // Fix: Set only U, not B
                self.push_stack(memory, processor_status);
            },
            Mnemonic::PLA => {
                let new_accumulator = self.pull_stack(memory);
                self.set_a(new_accumulator);
                self.update_zero_and_negative_flags(new_accumulator);
            }
            Mnemonic::PLP => {
                let status = (self.pull_stack(memory) & 0b11001111) | 0b00100000; // Fix: Ensure U is set
                self.set_s(status);
            },
            _ => return,
        }
    }
}
