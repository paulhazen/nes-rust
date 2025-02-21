

use crate::cpu::status_register::StatusRegister;
use crate::define_instruction;
use crate::cpu::CPU;

define_instruction!(CLD, |cpu: &mut CPU, _, _ :u8| {
    cpu.processor_status.set(StatusRegister::DECIMAL);
});
