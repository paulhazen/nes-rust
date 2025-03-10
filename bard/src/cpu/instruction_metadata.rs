//! # instruction_metadata.rs
//!
//! **Author:** Paul Hazen
//! **Created:** 2025-03-10
//! **License:** MIT (see LICENSE file)
//!
//! ## Description
//! Contains the struct definition for InstructionMetadata

use super::AddressingMode;
use super::Mnemonic;

#[derive(Clone)]
pub struct InstructionMetadata {
    pub mnemonic: Mnemonic,
    pub opcode: u8,
    pub size: u8,
    pub cycle_count: u8,
    pub addressing_mode: AddressingMode,
}

impl InstructionMetadata {
    pub fn debug_instruction_metadata(&self) {
        println!(
            "[{:?}] Opcode: 0x{:02X}, Size: {}, Cycles: {}, Mode: {:?}",
            self.mnemonic, self.opcode, self.size, self.cycle_count, self.addressing_mode, 
        );
    }
}