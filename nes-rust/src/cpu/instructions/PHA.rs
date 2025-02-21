use crate::cpu::CPU;
use crate::cpu::status_register::StatusRegister;
use crate::memory::MemoryBus;
use crate::define_instruction;

define_instruction!(PHA, |cpu: &mut CPU, memory: &mut MemoryBus, _: u8| {
    cpu.push_stack(memory, cpu.get_accumulator());
});

