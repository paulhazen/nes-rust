// INC, DEC, INX, DEX, INY, DEY
use crate::cpu::CPU;
use crate::memory::Bus;
use crate::memory::CPUBus;
use crate::define_instruction;

// Increment Memory (INC)
define_instruction!(INC, |cpu: &mut CPU, _memory: &mut CPUBus, mut value: u8| {
    value = value.wrapping_add(1);
    cpu.update_zero_and_negative_flags(value);
    cpu.set_a(value);
});

// Decrement Memory (DEC)
define_instruction!(DEC, |cpu: &mut CPU, _memory: &mut CPUBus, mut value: u8| {
    value = value.wrapping_sub(1);
    cpu.update_zero_and_negative_flags(value);
    cpu.set_a(value);
});

// Increment X Register (INX)
define_instruction!(INX, |cpu: &mut CPU, _: &mut CPUBus, _: u8| {
    cpu.set_x(cpu.get_x().wrapping_add(1));
    cpu.update_zero_and_negative_flags(cpu.get_x());
});

// Increment Y Register (INY)
define_instruction!(INY, |cpu: &mut CPU, _: &mut CPUBus, _: u8| {
    cpu.set_y(cpu.get_y().wrapping_add(1));
    cpu.update_zero_and_negative_flags(cpu.get_y());
});

// Decrement X Register (DEX)
define_instruction!(DEX, |cpu: &mut CPU, _: &mut CPUBus, _: u8| {
    let new_x = cpu.get_x().wrapping_sub(1);
    cpu.set_x(new_x);
    cpu.update_zero_and_negative_flags(new_x);
});

// Decrement Y Register (DEY)
define_instruction!(DEY, |cpu: &mut CPU, _: &mut CPUBus, _: u8| {
    let new_y = cpu.get_y().wrapping_sub(1);
    cpu.set_y(new_y);
    cpu.update_zero_and_negative_flags(new_y);
});
