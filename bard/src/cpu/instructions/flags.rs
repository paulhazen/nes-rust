// CLC, SEC, CLI, SEI, CLV, CLD, SED

use crate::cpu::instruction_mnemonic::InstructionMnemonic;
use crate::cpu::CPU;
use crate::cpu::Status;

impl CPU {
    pub fn handle_flags(&mut self, mnemonic: &InstructionMnemonic) {
        match mnemonic {
            InstructionMnemonic::CLC => self.set_flag(Status::CARRY, false),
            InstructionMnemonic::SEC => self.set_flag(Status::CARRY, true),
            InstructionMnemonic::CLI => self.set_flag(Status::INTERRUPT_DISABLE, false),
            InstructionMnemonic::SEI => self.set_flag(Status::INTERRUPT_DISABLE, true),
            InstructionMnemonic::CLV => self.set_flag(Status::OVERFLOW, false),
            InstructionMnemonic::CLD => self.set_flag(Status::DECIMAL, false),
            InstructionMnemonic::SED => self.set_flag(Status::CARRY, true),

            // Empty match arm to satisfy compiler
            _ => {},
        }
    }
}