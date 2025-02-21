use crate::cpu::CPU;
use crate::cpu::status_register::StatusRegister;
use crate::memory::MemoryBus;
use crate::define_instruction;

define_instruction!(NOP, |_: &mut CPU, _: &mut MemoryBus, _: u8| {});
