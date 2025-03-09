// CLC, SEC, CLI, SEI, CLV, CLD, SED

use crate::cpu::mnemonic::Mnemonic;
use crate::cpu::CPU;
use crate::cpu::Status;

impl CPU {
    pub fn handle_flags(&mut self, mnemonic: &Mnemonic) {
        match mnemonic {
            Mnemonic::CLC => self.set_flag(Status::CARRY, false),
            Mnemonic::SEC => self.set_flag(Status::CARRY, true),
            Mnemonic::CLI => self.set_flag(Status::INTERRUPT_DISABLE, false),
            Mnemonic::SEI => self.set_flag(Status::INTERRUPT_DISABLE, true),
            Mnemonic::CLV => self.set_flag(Status::OVERFLOW, false),
            Mnemonic::CLD => self.set_flag(Status::DECIMAL, false),
            Mnemonic::SED => self.set_flag(Status::CARRY, true),

            // Empty match arm to satisfy compiler
            _ => {},
        }
    }
}