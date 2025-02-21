#[macro_export]
macro_rules! opcode_entry {
    ($map:ident, $hex:expr, $mnemonic:ident, $size:expr, $cycles:expr, $mode:ident) => {
        ::paste::paste! {
            $map.insert(
                $hex as u8,
                OpCode {
                    mnemonic: crate::cpu::InstructionMnemonic::$mnemonic,
                    mode: crate::cpu::AddressingMode::$mode,
                    size: $size,
                    cycles: $cycles,
                    factory: || Box::new(crate::cpu::instructions::[<$mnemonic>]), // Explicit module path
                },
            );
        }
    };
}

#[macro_export]
macro_rules! define_instruction {
    ($name:ident, $execute_fn:expr) => {
        pub struct $name;

        impl Instruction for $name {
            #[inline(always)]
            fn execute(&self, cpu: &mut CPU, opcode: &OpCode, memory: &mut crate::memory::MemoryBus) {
                let value = match opcode.mode {
                    AddressingMode::Immediate => cpu.fetch_immediate(memory),
                    AddressingMode::ZeroPage  => cpu.fetch_zero_page(memory),
                    AddressingMode::ZeroPageX => cpu.fetch_zero_page_x(memory),
                    AddressingMode::Absolute  => cpu.fetch_absolute(memory),
                    AddressingMode::AbsoluteX => cpu.fetch_absolute_x(memory),
                    AddressingMode::AbsoluteY => cpu.fetch_absolute_y(memory),
                    AddressingMode::IndirectX => cpu.fetch_indirect_x(memory),
                    AddressingMode::IndirectY => cpu.fetch_indirect_y(memory),
                    _ => panic!(concat!(stringify!($name), " does not support addressing mode: {:?}"), opcode.mode),
                };

                $execute_fn(cpu, value);
            }
        }
    };
}