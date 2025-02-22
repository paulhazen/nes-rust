use super::AddressingMode;
use super::InstructionMnemonic;

pub struct InstructionMetadata {
    mnemonic: InstructionMnemonic,
    opcode: u8,
    size: u8,
    cycle_count: u8,
    addressing_mode: AddressingMode,
}