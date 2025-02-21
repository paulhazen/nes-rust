use crate::cpu::CPU;
use crate::cpu::status_register::StatusRegister;
use crate::memory::MemoryBus;
use crate::define_instruction;

define_instruction!(PHP, |cpu: &mut CPU, memory: &mut MemoryBus, _: u8| {
    let processor_status = cpu.processor_status.get();
    cpu.push_stack(memory, processor_status);
});
