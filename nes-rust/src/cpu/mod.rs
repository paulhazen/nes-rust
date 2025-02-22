mod addressing_mode;
mod cpu;
mod instruction_mnemonic;
mod opcode;
mod status_register;
mod instruction;
mod instructions;
mod opcode_table;
mod decoded_instruction;
mod instruction_metadata;


#[macro_use]
pub mod macros;

use status_register::Status;
use instruction_metadata::InstructionMetadata;
use instruction_mnemonic::InstructionMnemonic;
use addressing_mode::AddressingMode;
pub use cpu::CPU;