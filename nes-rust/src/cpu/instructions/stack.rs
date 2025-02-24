// PHA, PHP, PLA, PLP, TSX, TXS
use crate::define_instruction;
use crate::cpu::CPU;
use crate::memory::MemoryBus;

// Push Accumulator onto Stack (PHA)
define_instruction!(PHA, |cpu: &mut CPU, memory: &mut MemoryBus, _: u8| {
    cpu.push_stack(memory, cpu.get_a());
});

// Push Processor Status onto Stack (PHP)
define_instruction!(PHP, |cpu: &mut CPU, memory: &mut MemoryBus, _: u8| {
    let processor_status = cpu.get_s() | 0b00110000; // Set B and unused bits
    cpu.push_stack(memory, processor_status);
});

// Pull Accumulator from Stack (PLA)
define_instruction!(PLA, |cpu: &mut CPU, memory: &mut MemoryBus, _: u8| {
    let new_accumulator = cpu.pull_stack(memory);
    cpu.set_a(new_accumulator);
    cpu.update_zero_and_negative_flags(new_accumulator);
});

// Pull Processor Status from Stack (PLP)
define_instruction!(PLP, |cpu: &mut CPU, memory: &mut MemoryBus, _: u8| {
    let status = cpu.pull_stack(memory) & 0b11001111; // Ignore B and unused bits
    cpu.set_s(status);
});

// Transfer Stack Pointer to X (TSX)
define_instruction!(TSX, |cpu: &mut CPU, _, _ :u8| {
    let stack_pointer = cpu.get_s();
    cpu.set_x(stack_pointer);
    cpu.update_zero_and_negative_flags(stack_pointer);
});

// Transfer X to Stack Pointer (TXS)
define_instruction!(TXS, |cpu: &mut CPU, _, _ :u8| {
    cpu.set_s(cpu.get_x());
});
