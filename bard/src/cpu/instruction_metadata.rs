use super::AddressingMode;
use super::InstructionMnemonic;
use super::Instruction;

pub type InstructionFactory = fn() -> Box<dyn Instruction + 'static>;

#[derive(Clone)]
pub struct InstructionMetadata {
    pub mnemonic: InstructionMnemonic,
    pub opcode: u8,
    pub size: u8,
    pub cycle_count: u8,
    pub addressing_mode: AddressingMode,
    pub factory: InstructionFactory,
}

impl InstructionMetadata {
    pub fn debug_instruction_metadata(&self) {
        println!(
            "[{:?}] Opcode: 0x{:02X}, Size: {}, Cycles: {}, Mode: {:?}",
            self.mnemonic, self.opcode, self.size, self.cycle_count, self.addressing_mode, 
        );
    }
}