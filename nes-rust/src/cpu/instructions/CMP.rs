use crate::cpu::CPU;
use crate::cpu::status_register::StatusRegister;
use crate::memory::MemoryBus;
use crate::define_instruction;

define_instruction!(CMP, |cpu: &mut CPU, _: &mut MemoryBus, value: u8| {
    let result = cpu.get_accumulator().wrapping_sub(value);
    if cpu.get_accumulator() >= value {
        cpu.processor_status.set(StatusRegister::CARRY);
    } else {
        cpu.processor_status.clear(StatusRegister::CARRY);
    }
    cpu.update_zero_and_negative_flags(result);
});
