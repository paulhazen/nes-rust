// JMP, JSR
use crate::cpu::CPU;
use crate::memory::CPUBus;

impl CPU {
    pub fn handle_jump(&mut self, operand: &u8) {
        self.set_pc(*operand as u16);
    }

    pub fn handle_jump_to_subroutine(&mut self, operand: &u8, memory: &mut CPUBus) {
        let pc = self.get_pc().wrapping_sub(1);

        self.push_stack(memory, (pc >> 8) as u8);
        self.push_stack(memory, (pc & 0xFF) as u8);

        self.set_pc(*operand as u16);
    }
}