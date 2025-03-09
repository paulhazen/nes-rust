
use crate::cpu::InstructionMetadata;
use crate::cpu::AddressingMode;
use crate::cpu::CPU;
use crate::memory::CPUBus;

use super::mnemonic::Mnemonic;
use super::opcode_table::OPCODE_TABLE;

impl CPU {

    fn get_instruction_operand(&mut self, instruction_metadata: &InstructionMetadata, memory: &mut CPUBus) -> u8 {
        match instruction_metadata.addressing_mode {
            crate::cpu::AddressingMode::Relative => self.fetch_relative(memory),
            crate::cpu::AddressingMode::Immediate => self.fetch_immediate(memory),
            crate::cpu::AddressingMode::ZeroPage  => self.fetch_zero_page(memory),
            crate::cpu::AddressingMode::ZeroPageX => self.fetch_zero_page_x(memory),
            crate::cpu::AddressingMode::ZeroPageY => self.fetch_zero_page_y(memory),
            crate::cpu::AddressingMode::Absolute  => self.fetch_absolute(memory),
            crate::cpu::AddressingMode::AbsoluteX => self.fetch_absolute_x(memory),
            crate::cpu::AddressingMode::AbsoluteY => self.fetch_absolute_y(memory),
            crate::cpu::AddressingMode::Indirect => self.fetch_indirect(memory),
            crate::cpu::AddressingMode::IndirectX => self.fetch_indirect_x(memory),
            crate::cpu::AddressingMode::IndirectY => self.fetch_indirect_y(memory) ,
            crate::cpu::AddressingMode::Implied   => 0x00,
            crate::cpu::AddressingMode::Accumulator => self.get_a(),
        }
    }

    fn get_instruction_metadata(opcode: &u8) -> Result<&'static InstructionMetadata, String> {
        if let Some(instruction_metadata) = OPCODE_TABLE.get(&opcode) {
            Ok(instruction_metadata)
        } else {
            Err(format!("Unrecognized opcode \"{:>3}\"", opcode))
        }
    }
    
    pub fn execute_instruction(&mut self, opcode: &u8, memory: &mut CPUBus) -> u8 {

        // Retrieve the result of getting the instruction metadata for the opcode given
        let get_instruction_metadata_result = Self::get_instruction_metadata(opcode);

        // If the result was an error, then print the error and stop here
        if let Err(err) = get_instruction_metadata_result {
            println!("{}", err);
            return 0;
        }

        // Unwrap the instruction metadata now that we know the result was not an error.
        let instruction_metadata = get_instruction_metadata_result.unwrap();

        // Retrieve the address for the instruction
        let operand = self.get_instruction_operand(instruction_metadata, memory);

        // Dispatch the opcode to the right place
        match instruction_metadata.mnemonic {

            // region: Arithmetic
            
            Mnemonic::ADC | Mnemonic::CMP | 
            Mnemonic::CPX | Mnemonic::CPY | 
            Mnemonic::SBC => 
                self.handle_arithmetic(&operand, &instruction_metadata.mnemonic),

            // endregion

            // region: Bitwise 

            Mnemonic::AND | Mnemonic::ORA |
            Mnemonic::EOR | Mnemonic::BIT => 
                self.handle_bitwise(&operand, &instruction_metadata.mnemonic),

            // endregion

            // region: Branching

            Mnemonic::BEQ | Mnemonic::BNE |
            Mnemonic::BCS | Mnemonic::BCC |
            Mnemonic::BMI | Mnemonic::BPL |
            Mnemonic::BVC | Mnemonic::BVS => 
                self.handle_branching(&operand, &instruction_metadata.mnemonic, memory),

            // endregion

            // region: Flags

            Mnemonic::CLC | Mnemonic::SEC |
            Mnemonic::CLI | Mnemonic::SEI |
            Mnemonic::CLV | Mnemonic::CLD |
            Mnemonic::SED => self.handle_flags(&instruction_metadata.mnemonic),
            
            // endregion
            
            // region: Increment & Decrement

            Mnemonic::INC | Mnemonic::DEC =>
                self.handle_memory_increment_and_decrement(&operand, &instruction_metadata.mnemonic, memory),

            Mnemonic::INX | Mnemonic::DEX | 
            Mnemonic::INY | Mnemonic::DEY => 
                self.handle_register_increment_and_decrement(&instruction_metadata.mnemonic),

            // endregion
            
            // region: Jumps

            Mnemonic::JMP => self.handle_jump(&operand),
            Mnemonic::JSR => self.handle_jump_to_subroutine(&operand, memory),

            // endregion

            // region: Returns

            Mnemonic::RTI |
            Mnemonic::RTS => self.handle_return(&instruction_metadata.mnemonic, memory),

            // endregion

            // region: Load

            Mnemonic::LDA | Mnemonic::LDX |
            Mnemonic::LDY => self.handle_load(&operand, &instruction_metadata.mnemonic),

            // endregion

            // region: Store
            
            Mnemonic::STA | Mnemonic::STX |
            Mnemonic::STY => self.handle_store(&operand, &instruction_metadata.mnemonic, memory),

            // endregion

            // region: Shifts

            Mnemonic::ASL |
            Mnemonic::LSR |
            Mnemonic::ROL |
            Mnemonic::ROR => {
                // For shifts, if the adressing mode is accumulator, then don't pass the
                // operand, thus indicating that the accumulator should be udpated
                let adjusted_address = if instruction_metadata.addressing_mode == AddressingMode::Absolute {
                    None
                } else {
                    Some(operand as u16)
                };

                self.handle_shift(adjusted_address, &instruction_metadata.mnemonic, memory);
            }

            // endregion

            // region: Misc

            Mnemonic::NOP => self.handle_nop(),
            Mnemonic::BRK => self.handle_brk(memory),
            
            // endregion

            // region: Stack

            Mnemonic::PHA | Mnemonic::PHP |
            Mnemonic::PLA | Mnemonic::PLP 
                => self.handle_stack(&instruction_metadata.mnemonic, memory),

            // endregion
            
            // region: Transfer

            Mnemonic::TXA | Mnemonic::TYA |
            Mnemonic::TAY | Mnemonic::TSX |
            Mnemonic::TXS | Mnemonic::TAX 
                => self.handle_transfer(&instruction_metadata.mnemonic),

            // endregion
        };

        // TODO: This is not cycle accurate - because the cycle count here is a
        //       baseline, often this number is +1 if a page boundary has been 
        //       crossed. For the time being, this discrepency will make the 
        //       emulator not cycle-accurate.
        instruction_metadata.cycle_count
    }
}