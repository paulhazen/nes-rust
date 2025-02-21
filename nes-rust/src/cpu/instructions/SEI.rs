use crate::define_instruction;
use crate::cpu::CPU;
use crate::cpu::status_register::StatusRegister;

define_instruction!(SEI, |cpu: &mut CPU, _, _: u8| {
    cpu.processor_status.clear(StatusRegister::INTERRUPT_DISABLE);
});
