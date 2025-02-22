use crate::cpu::CPU;
use crate::cpu::Status;
use crate::memory::MemoryBus;
use crate::define_instruction;

define_instruction!(CMP, |cpu: &mut CPU, _: &mut MemoryBus, value: u8| {
    cpu.set_flag(Status::CARRY, cpu.get_a() >= value);
    let result = cpu.get_a().wrapping_sub(value);
    cpu.update_zero_and_negative_flags(result);
});
