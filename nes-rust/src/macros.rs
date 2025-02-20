#[macro_export]
macro_rules! hex {
    ($val:literal) => {
        u8::from_str_radix(concat!("0x", stringify!($val)), 16).unwrap()
    };
}

#[macro_export]
macro_rules! define_instruction {
    ($name:ident, $execute_fn:expr) => {
        pub struct $name;

        #[inline(always)]
        impl Instruction for $name {
            fn execute(&self, cpu: &mut CPU, opcode: &OpCode) {
                let value = match opcode.mode {
                    AddressingMode::Immediate => cpu.fetch_immediate(),
                    AddressingMode::ZeroPage => cpu.fetch_zero_page(),
                    AddressingMode::ZeroPageX => cpu.fetch_zero_page_x(),
                    AddressingMode::Absolute => cpu.fetch_absolute(),
                    AddressingMode::AbsoluteX => cpu.fetch_absolute_x(),
                    AddressingMode::AbsoluteY => cpu.fetch_absolute_y(),
                    AddressingMode::IndirectX => cpu.fetch_indirect_x(),
                    AddressingMode::IndirectY => cpu.fetch_indirect_y(),
                    _ => panic!(concat!(stringify!($name), " does not support addressing mode: {:?}"), opcode.mode),
                };

                $execute_fn(cpu, value);
            }
        }
    };
}
