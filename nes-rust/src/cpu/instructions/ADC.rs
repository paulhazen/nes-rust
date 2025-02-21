use crate::cpu::CPU;
use crate::cpu::status_register::StatusRegister;
use crate::memory::MemoryBus;
use crate::define_instruction;

define_instruction!(ADC, |cpu: &mut CPU, _: &mut MemoryBus, value: u8| {
    let mut result = cpu.get_accumulator() as u16;
    result += value as u16;
    result += cpu.processor_status.is_set(StatusRegister::CARRY) as u16;

    if result > 0xFF {
        cpu.processor_status.set(StatusRegister::CARRY);
    } else {
        cpu.processor_status.clear(StatusRegister::CARRY);
    }

    if ((cpu.get_accumulator() ^ value) & 0x80 == 0) && ((cpu.get_accumulator() ^ result as u8) & 0x80 != 0) {
        cpu.processor_status.set(StatusRegister::OVERFLOW);
    } else {
        cpu.processor_status.clear(StatusRegister::OVERFLOW);
    }

    cpu.set_accumulator(result as u8);
    cpu.update_zero_and_negative_flags(cpu.get_accumulator());
});
