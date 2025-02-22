use crate::cpu::CPU;
use crate::memory::MemoryBus;
use crate::define_instruction;

define_instruction!(JSR, |cpu: &mut CPU, memory: &mut MemoryBus, address: u16| {
    let pc = cpu.get_pc().wrapping_sub(1);
    
    // Push high byte
    //cpu.push_stack(memory, (pc >> 8) as u8);
    // Push low byte
    //cpu.push_stack(memory, (pc & 0xFF) as u8);

    // Jump to new address
    cpu.set_pc(address);
});
