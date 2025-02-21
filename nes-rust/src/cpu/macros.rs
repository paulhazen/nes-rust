
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

        impl crate::cpu::instruction::Instruction for $name {
            #[inline(always)]
            fn execute(&self, cpu: &mut CPU, opcode: &crate::cpu::opcode::OpCode, memory: &mut crate::memory::MemoryBus) {
                let value =  match opcode.mode {
                    crate::cpu::AddressingMode::Relative => cpu.fetch_relative(memory),
                    crate::cpu::AddressingMode::Immediate => cpu.fetch_immediate(memory),
                    crate::cpu::AddressingMode::ZeroPage  => cpu.fetch_zero_page(memory),
                    crate::cpu::AddressingMode::ZeroPageX => cpu.fetch_zero_page_x(memory),
                    crate::cpu::AddressingMode::ZeroPageY => cpu.fetch_zero_page_y(memory),
                    crate::cpu::AddressingMode::Absolute  => cpu.fetch_absolute(memory),
                    crate::cpu::AddressingMode::AbsoluteX => cpu.fetch_absolute_x(memory),
                    crate::cpu::AddressingMode::AbsoluteY => cpu.fetch_absolute_y(memory),
                    crate::cpu::AddressingMode::Indirect => cpu.fetch_indirect(memory),
                    crate::cpu::AddressingMode::IndirectX => cpu.fetch_indirect_x(memory),
                    crate::cpu::AddressingMode::IndirectY => cpu.fetch_indirect_y(memory) ,
                    crate::cpu::AddressingMode::Implied   => 0x00,
                    crate::cpu::AddressingMode::Accumulator => cpu.get_accumulator(),
                };
                

                $execute_fn(cpu, memory, value.try_into().unwrap());
            }
        }
    };
}