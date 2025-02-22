use crate::cpu::CPU;
use crate::cpu::Status;
use crate::memory::MemoryBus;
use crate::define_instruction;

define_instruction!(BIT, |cpu: &mut CPU, _: &mut MemoryBus, value: u8| {
    cpu.set_flag(Status::ZERO, cpu.get_a() & value == 0);
    cpu.set_flag(Status::OVERFLOW, value & 0x40 != 0);
    cpu.set_flag(Status::NEGATIVE, value & 0x80 != 0);
});
