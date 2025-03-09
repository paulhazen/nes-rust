use crate::cpu::mnemonic::Mnemonic;
use crate::cpu::CPU;
use crate::memory::{Bus, CPUBus};

impl CPU {

    pub fn handle_register_increment_and_decrement(&mut self, mnemonic: &Mnemonic) {
        match mnemonic {
            Mnemonic::INX => modify_value(self, Self::get_x, Self::set_x, |v| v.wrapping_add(1)),
            Mnemonic::DEX => modify_value(self, Self::get_x, Self::set_x, |v| v.wrapping_sub(1)),
            Mnemonic::INY => modify_value(self, Self::get_y, Self::set_y, |v| v.wrapping_add(1)),
            Mnemonic::DEY => modify_value(self, Self::get_y, Self::set_y, |v| v.wrapping_sub(1)),
            _ => {},
        }
    }

    pub fn handle_memory_increment_and_decrement(&mut self, address: &u8, mnemonic: &Mnemonic, memory: &mut CPUBus) {
        match mnemonic {
            Mnemonic::INC => modify_memory(self, address, |v| v.wrapping_add(1), memory),
            Mnemonic::DEC => modify_memory(self, address, |v| v.wrapping_sub(1), memory),
            _ => {},
        }
    }
}

/// Generalized function to increment or decrement a value.
fn modify_value<F>(cpu: &mut CPU, get: impl Fn(&CPU) -> u8, set: impl Fn(&mut CPU, u8), op: F)
where
    F: Fn(u8) -> u8,
{
    let new_value = op(get(cpu));
    set(cpu, new_value);
    cpu.update_zero_and_negative_flags(new_value);
}

/// Generalized function to modify memory at an address.
fn modify_memory<F>(cpu: &mut CPU, operand: &u8, op: F, memory: &mut CPUBus)
    where
        F: Fn(u8) -> u8,
    {
        let address = *operand as u16;
        let value = memory.read_byte(address);
        let new_value = op(value);
        memory.write_byte(address, new_value);
        cpu.update_zero_and_negative_flags(new_value);
    }

