use crate::cpu::CPU;
use crate::cpu::status_register::StatusRegister;
use crate::memory::MemoryBus;
use crate::define_instruction;

define_instruction!(BIT, |cpu: &mut CPU, _: &mut MemoryBus, value: u8| {
    if cpu.get_accumulator() & value == 0 {
        cpu.processor_status.set(StatusRegister::ZERO);
    } else {
        cpu.processor_status.clear(StatusRegister::ZERO);
    }

    if value & 0x40 != 0 {
        cpu.processor_status.set(StatusRegister::OVERFLOW);
    } else {
        cpu.processor_status.clear(StatusRegister::OVERFLOW);
    }

    if value & 0x80 != 0 {
        cpu.processor_status.set(StatusRegister::NEGATIVE);
    } else {
        cpu.processor_status.clear(StatusRegister::NEGATIVE);
    }
});
