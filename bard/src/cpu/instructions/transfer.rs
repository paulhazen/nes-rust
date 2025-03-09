use crate::cpu::instruction_mnemonic::InstructionMnemonic;
use crate::cpu::CPU;

/// Local helper function to transfer a value between registers and update flags.
fn transfer_and_update<F, G>(get: F, set: G, cpu: &mut CPU)
where
    F: Fn(&CPU) -> u8,    // Function to get the value
    G: Fn(&mut CPU, u8),  // Function to set the value
{
    let value = get(cpu);
    set(cpu, value);
    cpu.update_zero_and_negative_flags(value);
}

impl CPU {
    pub fn handle_transfer(&mut self, mnemonic: &InstructionMnemonic) {
        match mnemonic {
            InstructionMnemonic::TXA => transfer_and_update(Self::get_x, Self::set_a, self),
            InstructionMnemonic::TYA => transfer_and_update(Self::get_y, Self::set_a, self),
            InstructionMnemonic::TAY => transfer_and_update(Self::get_a, Self::set_y, self),
            InstructionMnemonic::TAX => transfer_and_update(Self::get_a, Self::set_x, self),
            InstructionMnemonic::TSX => transfer_and_update(Self::get_s, Self::set_x, self),
            InstructionMnemonic::TXS => self.set_s(self.get_x()), // TXS does NOT update flags
            _ => return,
        }
    }
}
