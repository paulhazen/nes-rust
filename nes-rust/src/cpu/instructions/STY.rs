use crate::cpu::CPU;
use crate::memory::MemoryBus;
use crate::define_instruction;

define_instruction!(STY, |cpu: &mut CPU, memory: &mut MemoryBus, address : u16| {
    memory.write(address, cpu.get_y());
});
