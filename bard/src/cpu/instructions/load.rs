// LDA, LDX, LDY, STA, STX, STY

use crate::cpu::instruction_mnemonic::InstructionMnemonic;
use crate::cpu::CPU;

impl CPU {
    pub fn handle_load(&mut self, operand: &u8, mnemonic: &InstructionMnemonic) {
        let value = *operand;
        match mnemonic {
            InstructionMnemonic::LDA => {
                self.set_a(value);
            },
            InstructionMnemonic::LDX => {
                self.set_x(value);
            }
            InstructionMnemonic::LDY => {
                self.set_y(value);
            }
            _ => {}
        }

        self.update_zero_and_negative_flags(value);
    }
}