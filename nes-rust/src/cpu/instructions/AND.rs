use crate::cpu::CPU;
use crate::memory::MemoryBus;
use crate::define_instruction;

define_instruction!(AND, |cpu: &mut CPU, _: &mut MemoryBus, value: u8| {
    cpu.set_a(cpu.get_a() & value);
    cpu.update_zero_and_negative_flags(cpu.get_a());
});
