use crate::cpu::CPU;
use crate::cpu::status_register::StatusRegister;
use crate::memory::MemoryBus;
use crate::define_instruction;

define_instruction!(PLA, |cpu: &mut CPU, memory: &mut MemoryBus, _: u8| {
    let new_accumulator = cpu.pull_stack(memory);
    cpu.set_accumulator(new_accumulator);
    cpu.update_zero_and_negative_flags(cpu.get_accumulator());
});
