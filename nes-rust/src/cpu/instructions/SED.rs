use crate::cpu::CPU;
use crate::cpu::status_register::StatusRegister;
use crate::memory::MemoryBus;
use crate::define_instruction;

define_instruction!(SED, |cpu: &mut CPU, _: &mut MemoryBus, _: u8| {
    cpu.processor_status.set(StatusRegister::DECIMAL);
});
