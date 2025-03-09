// BEQ, BNE, BCS, BCC, BMI, BPL, BVC, BVS

use crate::cpu::{Status, CPU};
use crate::memory::CPUBus;
use crate::cpu::instruction_mnemonic::InstructionMnemonic;

impl CPU {
    pub fn handle_branching(&mut self, operand: &u8, mnemonic: &InstructionMnemonic, memory: &mut CPUBus) {
        match mnemonic {
            InstructionMnemonic::BEQ => conditional_branch(self, operand, self.is_flag_set(Status::ZERO), memory),
            InstructionMnemonic::BNE => conditional_branch(self, operand, !self.is_flag_set(Status::ZERO), memory),
            
            InstructionMnemonic::BCS => conditional_branch(self, operand, self.is_flag_set(Status::CARRY), memory),
            InstructionMnemonic::BCC => conditional_branch(self, operand, !self.is_flag_set(Status::CARRY), memory),
            
            InstructionMnemonic::BMI => conditional_branch(self, operand, self.is_flag_set(Status::NEGATIVE), memory),
            InstructionMnemonic::BPL => conditional_branch(self, operand, !self.is_flag_set(Status::NEGATIVE), memory),
            
            InstructionMnemonic::BVS => conditional_branch(self, operand, self.is_flag_set(Status::OVERFLOW), memory),
            InstructionMnemonic::BVC => conditional_branch(self, operand, !self.is_flag_set(Status::OVERFLOW), memory),
            
            // Empty match arm to satisfy compiler
            _ => {},
        }
    }
}

fn conditional_branch(cpu: &mut CPU, operand: &u8, condition: bool, memory: &mut CPUBus) {
    cpu.branch(memory, condition, *operand);
}