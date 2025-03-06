use super::InstructionMetadata;

pub struct DecodedInstruction<'a> {
    metadata: &'a InstructionMetadata,
    operands: [u8; 2],
    operand_size: u8,
    memory_address: u16,
}