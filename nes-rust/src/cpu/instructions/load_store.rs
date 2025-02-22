// LDA, LDX, LDY, STA, STX, STY

use crate::cpu::CPU;
use crate::memory::MemoryBus;
use crate::define_instruction;

define_instruction!(LDA, |cpu: &mut CPU, _, value| {
    cpu.set_a(value);
    cpu.update_zero_and_negative_flags(value);
}); 

define_instruction!(LDX, |cpu: &mut CPU, _, value| {
    cpu.set_x(value);
});

define_instruction!(LDY, |cpu: &mut CPU, _, value| {
    cpu.set_y(value);
});

define_instruction!(STA, |cpu: &mut CPU, memory: &mut MemoryBus, address : u16| {
    memory.write(address, cpu.get_a());
});

define_instruction!(STX, |cpu: &mut CPU, memory: &mut MemoryBus, address : u16| {
    memory.write(address, cpu.get_x());
});

define_instruction!(STY, |cpu: &mut CPU, memory: &mut MemoryBus, address : u16| {
    memory.write(address, cpu.get_y());
});