// LDA, LDX, LDY, STA, STX, STY

use crate::cpu::mnemonic::Mnemonic;
use crate::cpu::CPU;

impl CPU {
    pub fn handle_load(&mut self, operand: &u8, mnemonic: &Mnemonic) {
        let value = *operand;
        match mnemonic {
            Mnemonic::LDA => {
                self.set_a(value);
            },
            Mnemonic::LDX => {
                self.set_x(value);
            }
            Mnemonic::LDY => {
                self.set_y(value);
            }
            _ => {}
        }

        self.update_zero_and_negative_flags(value);
    }
}