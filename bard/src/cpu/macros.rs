

#[macro_export]
macro_rules! instruction_metadata_entry {
    ($map:ident, $hex:expr, $mnemonic:ident, $size:expr, $cycles:expr, $mode:ident) => {
        ::paste::paste! {
            $map.insert(
                $hex as u8,
                crate::cpu::InstructionMetadata {
                    mnemonic: crate::cpu::InstructionMnemonic::$mnemonic,
                    addressing_mode: crate::cpu::AddressingMode::$mode,
                    opcode: $hex,
                    size: $size,
                    cycle_count: $cycles
                },
            );
        }
    };
}

/* 
#[macro_export]
macro_rules! create_execute_function {
    ($execute_fn:expr) => {
        |cpu: &mut CPU, bus: &mut crate::memory::CPUBus, operand: u8| {
            $execute_fn(cpu, bus, operand.into())
        }
    };
}

#[macro_export]
macro_rules! define_instruction {
    ($name:ident, $execute_fn:expr) => {
        pub struct $name;

        impl crate::cpu::instruction::Instruction for $name {
            
            fn execute(&self, cpu: &mut CPU, opcode: &crate::cpu::InstructionMetadata, memory: &mut crate::memory::CPUBus) -> u8 {

                let execute_function: fn(&mut CPU, &mut crate::memory::CPUBus, u8) = 
                    crate::create_execute_function!($execute_fn);

                let value =  match opcode.addressing_mode {
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
                    crate::cpu::AddressingMode::Accumulator => cpu.get_a(),
                };
                
                // Reset, and start counting cycles that may occur during execute function
                memory.set_cycle_counter(0);

                // Print the instruction metadata
                //opcode.debug_instruction_metadata();

                execute_function(cpu, memory, value.try_into().unwrap());

                // Retrieve the number of cycles that the memory bus recorded
                let _measured_cycles = memory.get_cycles();

                // Get the number of base cycles that the instruction is supposed to take
                let base_cycles = opcode.cycle_count;

                // TODO: Inspect circumstances where measured_cycles != base_cycles.

                base_cycles
            }
        }
    };
} */