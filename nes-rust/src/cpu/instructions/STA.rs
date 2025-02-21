use crate::cpu::CPU;
use crate::memory::MemoryBus;
use crate::define_instruction;


define_instruction!(STA, |cpu: &mut CPU, memory: &mut MemoryBus, address : u16| {
    memory.write(address, cpu.get_accumulator());
});