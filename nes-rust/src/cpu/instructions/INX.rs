use crate::cpu::CPU;
use crate::memory::MemoryBus;
use crate::define_instruction;

define_instruction!(INX, |cpu: &mut CPU, _: &mut MemoryBus, _: u8| {
    cpu.set_x(cpu.get_x().wrapping_add(1));
    cpu.update_zero_and_negative_flags(cpu.get_x());
});
