// AND, ORA, EOR, BIT

use crate::cpu::instruction_mnemonic::InstructionMnemonic;
use crate::cpu::CPU;
use crate::cpu::Status;

pub const OVERFLOW_BIT: u8 = 0x40;

impl CPU {
    pub fn handle_bitwise(&mut self, operand: &u8, mnemonic: &InstructionMnemonic) {
        match mnemonic {
            InstructionMnemonic::AND => {
                self.set_a(self.get_a() & operand);
                self.update_zero_and_negative_flags(self.get_a());
            },
            InstructionMnemonic::ORA => {
                self.set_a(self.get_a() | operand);
                self.update_zero_and_negative_flags(self.get_a());
            },
            InstructionMnemonic::EOR => {
                self.set_a(self.get_a() ^ operand);
                self.update_zero_and_negative_flags(self.get_a());
            }
            InstructionMnemonic::BIT => {
                self.set_flag(Status::ZERO, self.get_a() & operand == 0);
                self.set_flag(Status::OVERFLOW, operand & OVERFLOW_BIT != 0);
                self.set_flag(Status::NEGATIVE, operand & CPU::SIGN_BIT != 0);
            }
            _ => {}
        }
    }
}