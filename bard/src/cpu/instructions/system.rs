use crate::cpu::CPU;
use crate::memory::CPUBus;
use crate::memory::Bus;
use crate::define_instruction;

define_instruction!(NOP, |_: &mut CPU, _: &mut CPUBus, _: u8| {});

// Break (BRK)
define_instruction!(BRK, |cpu: &mut CPU, memory: &mut CPUBus, _: u8| {
    let pc = cpu.get_pc().wrapping_add(1); // Increment PC to simulate fetching next instruction
    
    // Push PC high and low bytes onto the stack
    cpu.push_stack(memory, (pc >> 8) as u8);
    cpu.push_stack(memory, (pc & 0xFF) as u8);

    // Push processor status with Break flag set
    let mut status = cpu.get_s();
    status |= 0b00110000; // Set B (Break) and Unused bit
    cpu.push_stack(memory, status);

    // Set Interrupt Disable flag
    cpu.set_flag(crate::cpu::Status::INTERRUPT_DISABLE, true);

    // Load new PC from IRQ/BRK vector ($FFFE/$FFFF)
    // TODO: Properly overwrite read_word write_word in CPUBus!!!!
    let low = memory.default_read_word(0xFFFE);
    let high = memory.default_read_word(0xFFFF);
    cpu.set_pc((high << 8) | low);
});
