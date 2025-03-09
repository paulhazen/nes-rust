// BEQ, BNE, BCS, BCC, BMI, BPL, BVC, BVS

use crate::cpu::{Status, CPU};
use crate::memory::CPUBus;
use crate::cpu::mnemonic::Mnemonic;

impl CPU {
    pub fn handle_branching(&mut self, operand: &u8, mnemonic: &Mnemonic, memory: &mut CPUBus) {
        match mnemonic {
            Mnemonic::BEQ => conditional_branch(self, operand, self.is_flag_set(Status::ZERO), memory),
            Mnemonic::BNE => conditional_branch(self, operand, !self.is_flag_set(Status::ZERO), memory),
            
            Mnemonic::BCS => conditional_branch(self, operand, self.is_flag_set(Status::CARRY), memory),
            Mnemonic::BCC => conditional_branch(self, operand, !self.is_flag_set(Status::CARRY), memory),
            
            Mnemonic::BMI => conditional_branch(self, operand, self.is_flag_set(Status::NEGATIVE), memory),
            Mnemonic::BPL => conditional_branch(self, operand, !self.is_flag_set(Status::NEGATIVE), memory),
            
            Mnemonic::BVS => conditional_branch(self, operand, self.is_flag_set(Status::OVERFLOW), memory),
            Mnemonic::BVC => conditional_branch(self, operand, !self.is_flag_set(Status::OVERFLOW), memory),
            
            // Empty match arm to satisfy compiler
            _ => {},
        }
    }
}

fn conditional_branch(cpu: &mut CPU, operand: &u8, condition: bool, memory: &mut CPUBus) {
    cpu.branch(memory, condition, *operand);
}