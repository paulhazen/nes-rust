use crate::cpu::CPU;
use crate::memory::MemoryBus;
use crate::define_instruction;

define_instruction!(RTI, |cpu: &mut CPU, memory: &mut MemoryBus, _: u8| {
    let new_processor_status = cpu.pull_stack(memory);
    cpu.set_s(new_processor_status);
    let low = cpu.pull_stack(memory) as u16;
    let high = cpu.pull_stack(memory) as u16;
    cpu.set_pc((high << 8) | low);
});
