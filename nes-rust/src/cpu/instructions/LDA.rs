use crate::cpu::CPU;
use crate::instruction::Instruction;
use crate::opcode::OpCode;
use crate::cpu::addressing_mode::AddressingMode;

define_instruction!(LDA, |cpu, value| {
    cpu.accumulator = value;
    cpu.update_zero_and_negative_flags(value);
});
