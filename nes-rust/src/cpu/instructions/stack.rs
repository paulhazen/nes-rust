// PHA, PHP, PLA, PLP, TSX, TXS

use crate::define_instruction;
use crate::cpu::CPU;
use crate::memory::MemoryBus;

define_instruction!(PHA, |cpu: &mut CPU, memory: &mut MemoryBus, _: u8| {
    //cpu.push_stack(memory, cpu.get_a());
});

define_instruction!(PHP, |cpu: &mut CPU, memory: &mut MemoryBus, _: u8| {
    //let processor_status = cpu.get_pc();
    //cpu.push_stack(memory, processor_status);
});

define_instruction!(PLA, |cpu: &mut CPU, memory: &mut MemoryBus, _: u8| {
    let new_accumulator = cpu.pull_stack(memory);
    cpu.set_a(new_accumulator);
    cpu.update_zero_and_negative_flags(cpu.get_a());
});

define_instruction!(TSX, |cpu: &mut CPU, _, _ :u8| {
    cpu.set_x(cpu.get_s());
});

define_instruction!(TXS, |cpu: &mut CPU, _, _ :u8| {
    cpu.set_s(cpu.get_x());
});