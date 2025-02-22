// JMP, JSR, RTS, RTI

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

define_instruction!(RTS, |cpu: &mut CPU, memory: &mut MemoryBus, _: u8| {
    let low = cpu.pull_stack(memory) as u16;
    let high = cpu.pull_stack(memory) as u16;
    cpu.set_pc(((high << 8) | low).wrapping_add(1));
});

define_instruction!(RTI, |cpu: &mut CPU, memory: &mut MemoryBus, _: u8| {
    let new_processor_status = cpu.pull_stack(memory);
    cpu.set_s(new_processor_status);
    let low = cpu.pull_stack(memory) as u16;
    let high = cpu.pull_stack(memory) as u16;
    cpu.set_pc((high << 8) | low);
});

