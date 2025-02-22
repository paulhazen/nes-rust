use crate::cpu::CPU;
use crate::memory::MemoryBus;
use crate::define_instruction;

define_instruction!(PHP, |cpu: &mut CPU, memory: &mut MemoryBus, _: u8| {
    //let processor_status = cpu.get_pc();
    //cpu.push_stack(memory, processor_status);
});
