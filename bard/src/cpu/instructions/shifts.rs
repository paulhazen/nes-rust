// ASL, LSR, ROL, ROR

use super::super::Status;
use super::super::CPU;
use crate::cpu::mnemonic::Mnemonic;
use crate::memory::Bus;
use crate::memory::CPUBus;

impl CPU {
    pub fn handle_shift(&mut self, address: Option<u16>, mnemonic: &Mnemonic, memory: &mut CPUBus) {
        let mut value = match address {
            Some(addr) => memory.read_byte(addr),
            None => self.get_a(),
        };

        let new_carry = match mnemonic {
            Mnemonic::ASL => {
                self.set_flag(Status::CARRY, value & 0x80 != 0);
                value <<= 1; // shift left
                value & 0x80 != 0
            },
            Mnemonic::LSR => {
                self.set_flag(Status::CARRY, value & 0x01 != 0);
                value >>= 1; // Shift right
                false // LSR always clears bit 7
            }
            Mnemonic::ROR => {
                let carry_in = (self.is_flag_set(Status::CARRY) as u8) << 7;
                let new_carry = value & 0x01 != 0;
                value = (value >> 1) | carry_in; // Shift right and insert carry
                new_carry
            },
            Mnemonic::ROL => {
                let carry_in = self.is_flag_set(Status::CARRY) as u8;
                let new_carry = value & 0x80 != 0;
                value = (value << 1) | carry_in; // Shift left and insert carry
                new_carry
            },
            // Empty arm to satisfy compiler
            _ => return,
        };

        self.set_flag(Status::CARRY, new_carry);
        self.update_zero_and_negative_flags(value);

        match address {
            Some(addr) => memory.write_byte(addr, value),
            None => {
                self.set_a(value);
                true // because write_byte returns bool based on whether the write succeeded, we need to return a bool here too.
            },
        };
    }
}