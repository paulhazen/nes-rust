// ADC, SBC, CMP, CPX, CPY

use crate::cpu::instruction_mnemonic::InstructionMnemonic;
use crate::cpu::CPU;
use crate::cpu::Status;

const BYTE_MASK: u16 = 0xFF;
const CARRY_THRESHOLD: u16 = 0x100;
const BORROW_ADJUSTMENT: u16 = 1;

impl CPU {
    pub fn handle_arithmetic(&mut self, operand: &u8, mnemonic: &InstructionMnemonic) {
        match mnemonic {
            // Add with carry
            InstructionMnemonic::ADC => add_with_carry(self, operand),
            
            // Subtract with carry
            InstructionMnemonic::SBC => subtract_with_carry(self, operand),

            // Compare accumulator register
            InstructionMnemonic::CMP => compare(self, operand, &self.get_a()),

            // Compare x register
            InstructionMnemonic::CPX => compare(self, operand, &self.get_x()),

            // Compare y register
            InstructionMnemonic::CPY => compare(self, operand, &self.get_y()),

            // Empty arm so compiler doesn't complain
            _ => {}
        }
    }
}

fn compare(cpu: &mut CPU, operand: &u8, compare_to: &u8) {
    cpu.set_flag(Status::CARRY, compare_to >= operand);
    let result = compare_to.wrapping_sub(*operand);
    cpu.update_zero_and_negative_flags(result);
}

fn adjust_with_carry(cpu: &mut CPU, operand: u8, is_subtract: bool) {
    let operand = if is_subtract { operand ^ BYTE_MASK as u8 } else { operand }; // 1's complement for subtraction
    let mut result = cpu.get_a() as u16;
    result += operand as u16;
    result += cpu.is_flag_set(Status::CARRY) as u16 - if is_subtract { BORROW_ADJUSTMENT } else { 0 }; // Adjust for SBC borrow

    cpu.set_flag(Status::CARRY, result >= CARRY_THRESHOLD);

    let is_overflow = ((cpu.get_a() ^ operand) & CPU::SIGN_BIT == 0) && ((cpu.get_a() ^ result as u8) & CPU::SIGN_BIT != 0);
    cpu.set_flag(Status::OVERFLOW, is_overflow);

    cpu.set_a(result as u8);
    cpu.update_zero_and_negative_flags(cpu.get_a());
}

fn add_with_carry(cpu: &mut CPU, operand: &u8) {
    adjust_with_carry(cpu, *operand, false);
}

fn subtract_with_carry(cpu: &mut CPU, operand: &u8) {
    adjust_with_carry(cpu, *operand, true);
}
