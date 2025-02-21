use crate::cpu::CPU;
use crate::cpu::status_register::StatusRegister;
use crate::memory::MemoryBus;
use crate::define_instruction;

define_instruction!(INY, |cpu: &mut CPU, _: &mut MemoryBus, _: u8| {
    cpu.set_y_register(cpu.get_y_register().wrapping_add(1));
    cpu.update_zero_and_negative_flags(cpu.get_y_register());
});
