// ADC, SBC, CMP, CPX, CPY

use crate::cpu::CPU;
use crate::cpu::Status;
use crate::memory::MemoryBus;
use crate::define_instruction;

define_instruction!(ADC, |cpu: &mut CPU, _: &mut MemoryBus, value: u8| {
    let mut result = cpu.get_a() as u16;
    result += value as u16;
    result += cpu.is_flag_set(Status::CARRY) as u16;

    cpu.set_flag(Status::CARRY, result > 0xFF);

    let is_overflow = ((cpu.get_a() ^ value) & 0x80 == 0) && ((cpu.get_a() ^ result as u8) & 0x80 != 0);

    cpu.set_flag(Status::OVERFLOW, is_overflow);

    cpu.set_a(result as u8);
    cpu.update_zero_and_negative_flags(cpu.get_a());
});

define_instruction!(CMP, |cpu: &mut CPU, _: &mut MemoryBus, value: u8| {
    cpu.set_flag(Status::CARRY, cpu.get_a() >= value);
    let result = cpu.get_a().wrapping_sub(value);
    cpu.update_zero_and_negative_flags(result);
});