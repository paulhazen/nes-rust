use crate::cpu::status_register::Status;
use crate::cpu::CPU;
use crate::memory::CPUBus;
use crate::memory::Bus;

impl CPU {
    pub fn handle_nop(&self) {
        // NOP does nothing on purpose.
    }

    pub fn handle_brk(&mut self, memory: &mut CPUBus) {
        let pc = self.get_pc().wrapping_add(1); // Increment PC to simulate fetching next instruction
        // Push PC high and low bytes onto the stack
        self.push_stack(memory, (pc >> 8) as u8);
        self.push_stack(memory, (pc & 0xFF) as u8);

        // Push processor status with Break flag set
        let mut status = self.get_s();
        status |= 0b00110000; // Set B (Break) and Unused bit
        self.push_stack(memory, status);

        // Set Interrupt Disable flag
        self.set_flag(Status::INTERRUPT_DISABLE, true);

        // Load new PC from IRQ/BRK vector ($FFFE/$FFFF)
        // TODO: Properly overwrite read_word write_word in CPUBus!!!!
        let low = memory.default_read_word(0xFFFE);
        let high = memory.default_read_word(0xFFFF);
        self.set_pc((high << 8) | low);
    }
}