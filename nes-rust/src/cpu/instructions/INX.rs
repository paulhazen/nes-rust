use crate::cpu::CPU;
use crate::cpu::status_register::StatusRegister;
use crate::memory::MemoryBus;
use crate::define_instruction;

define_instruction!(INX, |cpu: &mut CPU, _: &mut MemoryBus, _: u8| {
    cpu.set_x_register(cpu.get_x_register().wrapping_add(1));
    cpu.update_zero_and_negative_flags(cpu.get_x_register());
});
