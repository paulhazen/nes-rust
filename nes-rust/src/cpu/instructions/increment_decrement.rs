// INC, DEC, INX, DEX, INY, DEY
use crate::cpu::CPU;
use crate::memory::MemoryBus;
use crate::define_instruction;

define_instruction!(INX, |cpu: &mut CPU, _: &mut MemoryBus, _: u8| {
    cpu.set_x(cpu.get_x().wrapping_add(1));
    cpu.update_zero_and_negative_flags(cpu.get_x());
});

define_instruction!(INY, |cpu: &mut CPU, _: &mut MemoryBus, _: u8| {
    cpu.set_y(cpu.get_y().wrapping_add(1));
    cpu.update_zero_and_negative_flags(cpu.get_y());
});

define_instruction!(DEX, |cpu: &mut CPU, _: &mut MemoryBus, _: u8| {
    let new_x = cpu.get_x().wrapping_sub(1);
    cpu.set_x(new_x);
    cpu.update_zero_and_negative_flags(new_x);
});

define_instruction!(DEY, |cpu: &mut CPU, _: &mut MemoryBus, _: u8| {
    let new_y = cpu.get_y().wrapping_sub(1);
    cpu.set_y(new_y);
    cpu.update_zero_and_negative_flags(new_y);
});

//define_instruction!(INC, |cpu: &mut CPU, memory: &mut MemoryBus, address: u8| {
//    let addr = cpu.get_effective_address(address);
//    let value = memory.read(addr).wrapping_add(1);
//    memory.write(addr, value);
//    cpu.update_zero_and_negative_flags(value);
//});
//
//define_instruction!(DEC, |cpu: &mut CPU, memory: &mut MemoryBus, address: u8| {
//    let addr = cpu.get_effective_address(address);
//    let value = memory.read(addr).wrapping_sub(1);
//    memory.write(addr, value);
//    cpu.update_zero_and_negative_flags(value);
//});
