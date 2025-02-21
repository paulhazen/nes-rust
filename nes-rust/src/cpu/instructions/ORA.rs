use crate::cpu::CPU;
use crate::memory::MemoryBus;
use crate::define_instruction;

define_instruction!(ORA, |cpu: &mut CPU, _: &mut MemoryBus, value: u8| {
    cpu.set_accumulator(cpu.get_accumulator() | value);
    cpu.update_zero_and_negative_flags(cpu.get_accumulator());
});
