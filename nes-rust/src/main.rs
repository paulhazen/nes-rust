struct StatusRegister(u8);

/*
    Helper struct that makes it clear which bits are being set within the 
    processor status register of the 6502 CPU.
 */
impl StatusRegister {
    /*
        The carry flag is set if the last operation caused an overflow from 
        bit 8 of the result or an underflow from bit 0. This condition is set
         during arithmetic, comparison and during logical shifts. It can be 
         explicitly set using the 'Set Carry Flag' (SEC) instruction and 
         cleared with the 'Clear Carry Flag' (CLC).
     */
    const CARRY: u8 = 1 << 0;

    /*
        The zero flag is set if the result of the last operation was zero.
     */
    const ZERO: u8 = 1 << 1;

    /*
        The interrupt disable flag is set if the program has executed a 
        'Set Interrupt Disable' (SEI) instruction. While this flag is set the 
        processor will not respond to interrupts from devices until it is 
        cleared by a 'Clear Interrupt Disable' (CLI) instruction.
     */
    const INTERRUPT_DISABLE: u8 = 1 << 2;

    /*
        While the decimal mode flag is set the processor will obey the rules of
        Binary Coded Decimal (BCD) arithmetic during addition and subtraction. 
        The flag can be explicitly set using 'Set Decimal Flag' (SED) and 
        cleared with 'Clear Decimal Flag' (CLD).
     */
    const DECIMAL: u8 = 1 << 3;

    /*
        The break command bit is set when a BRK instruction has been executed 
        and an interrupt has been generated to process it.
     */
    const BREAK: u8 = 1 << 4;

    /*
        In the NES CPU (Obelisk 6502) this bit is always 1 for historical 
        purposes and compatibility.
     */
    const UNUSED: u8 = 1 << 5;

    /*
        The overflow flag is set during arithmetic operations if the result has
        yielded an invalid 2's complement result (e.g. addint to positive 
        numbers and ending up with a negative result: 64 + 64 => -128). It is
        determined by looking at the carry between bits 6 and 7 and between bit
        7 and the carry flag.
     */
    const OVERFLOW: u8 = 1 << 6;

    /*
        The negative flag is set if the result of the last operation had bit 7
        set to a one.
     */
    const NEGATIVE: u8 = 1 << 7;

    /*
        Note:

        The use of the #[inline(always)] attributes utilized below guarantees
        that these functions will be inlined by the rust compiler, meaning that
        there is little if no overhead to this implementation compared to using
        a raw u8 value to store the processor status.
     */

    // Clears a particular flag in the status register.
    #[inline(always)]
    fn clear(&mut self, flag: u8) {
        self.0 &= !flag;
    }

    // Determines if the indicated flag is set in the status register.
    #[inline(always)]
    fn is_set(&mut self, flag:u8) -> bool {
        self.0 & flag != 0
    }

    // Creates a new StatusRegister struct.
    #[inline(always)]
    fn new() -> Self {
        StatusRegister(Self::UNUSED) // Ensures that bit 5 is always 1
    }

    // Sets a particular flag in the status register.
    #[inline(always)]
    fn set(&mut self, flag: u8) {
        self.0 |= flag;
    }
}

struct CPU {
    accumulator: u8,                  // Accumulator
    x_register: u8,                   // "X" Register
    y_register: u8,                   // "Y" Register
    program_counter: u16,             // Program Counter
    stack_pointer: u8,                // Stack Pointer
    processor_status: StatusRegister, // Processor Status Register
}

impl CPU {
    fn new() -> Self {
        CPU {
            accumulator: 0, 
            x_register: 0,
            y_register: 0, 
            program_counter: 0x8000, // NES program entry point
            stack_pointer: 0xFD,
            processor_status: StatusRegister::new(),
        }
    }

    /**
     * Executes the given op code executor.
     */
    #[inline(always)]
    pub fn execute_instruction<T: Instruction>(&mut self, executor: OpCodeExecutor<T>) {
        executor.execute(self);
    }
}

pub trait Instruction {
    fn execute(&self, cpu: &mut CPU, opcode: &OpCode);
}

struct OpCode {
    mnemonic: InstructionMnemonic, 
    mode: AddressingMode,
    size: u8,
}

pub struct OpCodeExecutor<T: Instruction> {
    opcode: OpCode,
    executor: T,
}

impl<T: Instruction> OpCodeExecutor<T> {
    pub fn new(opcode: OpCode, executor: T) -> Self {
        Self {opcode, executor }
    }

    pub fn execute(&self, cpu: &mut CPU) {
        self.executor.execute(cpu, &self.opcode);
    }
}

enum AddressingMode {

    /*
        Instruction does not take an operand - it implicitly operates on a 
        register.

        Size: 1 byte (just the opcode).

        Used for register-based operations or simple control instructions.
     */
    Implied,

    /*
        Instruction includes operand directly as part of itself

        Size: 2 bytes (opcode + immediate value)

        Used when an instruction needs a constant value.
     */
    Immediate,

    /*
        Operand is a memory address located in the first 256 bytes of RAM.

        Size: 2 bytes (opcode + 8-bit address)

        Faster and more memory-efficient than absolute addressing.
     */
    ZeroPage,


    /*
        Similar to Zero Page, but adds the X register as an offset.

        Size: 2 bytes (opcode + 8-bit address)

        Useful for iterating over arrays or tables.
     */
    ZeroPageX,

    /*
        Similar to ZeroPageX, but uses Y register instead of X register.

        Size: 2 bytes (opcode + 8-bit address)

        Rarely used (X is preferred) but useful in some indexing cases.
        TODO: Determine what indexing cases specifically this is useful for.
     */
    ZeroPageY,

    /*
        Operand is a full 16-bit memory address

        Size: 3 bytes (opcode + 16-bit address)

        Used for accessing specific memory locations.
     */
    Absolute,

    /* 
        Similar to Absolute, but adds the X register as an offset.

        Size: 3 bytes (opcode + 16 bit address)

        Common for iterating over tables and graphics memory.
     */
    AbsoluteX,

    /*
        Like Absolute, but adds the Y register as an offset

        Size: 3 bytes (opcode + 16-bit address)

        Often used for reading sprite data or text buffers
     */
    AbsoluteY,

    /*
        Operand is a memory address that contains a pointer to another memory 
        address.

        Size: 3 bytes (opcode + 16-bit address)

        Only used for JMP, allowing dynamic code execution.

        NOTE: There is a bug in the 6502 processor - if the pointer crosses a
              page boundary (e.g. $12FF), the second byte is read from ($1200 
              instead of $1300).
     */
    IndirectAddressing,

    /*
        Operand is an address in the zero-page, which is indexec by X and 
        points to another address.

        Size: 2 bytes (opcode + 8-bit base address)

        Used for reading indexed tables stored in zero pages.
     */
    IndirectX,

    /*
        Operand is a pointer in zero-page but the fetched address is offset by 
        Y

        Size: 2 bytes (opcode + 8-bit base address)

        Used for scrolling and sprite tables.
     */
    IndirectY,

    /*
        Used for branch instructions (BEQ, BNE, etc.).

        Size: 2 bytes (opcode + signed 8-bit offset)

        Only used for branching.
     */
    Relative,

    /*
        The instruction operates directly on the Accumulator (A).

        Size: 1 byte (opcode only)

        Used for bit shifts and arithmetic directly on the A register.
     */
    Accumulator,   
}

enum InstructionMnemonic {

    ADC, AND, ASL, BCC, BCS, BEQ, BIT, BMI, BNE, BPL, BRK, BVC, BVS, CMP, CPX, 
    CPY, DEC, DEX, DEY, EOR, INC, INX, INY, JMP, JSR, LDA, LDX, LDY, LSR, NOP, 
    ORA, PHA, PHP, PLA, ROL, ROR, RTI, RTS, SBC, STA, STX, STY, TAX, TAY, TSX, 
    TXA, TXS, TYA,

    // region: ProcessorStatusInstructions
    
    SEC, // Set the carry bit of the process status register to 1.
    CLC, // Set the carry bit of the process status register to 0.
    
    SED, // Set the decimal mode bit of the process status register to 1.
    CLD, // Set the decimal mode bit of the process status register to 0.

    SEI, // Set the interrupt disable bit of the process status register to 1.
    CLI, // Set the interrupt disable bit of the process status register to 0.

    CLV, // Set the overflow bit of the process status register to 0.
    
    // endregion: ProcessorStatusInstructions
}

fn main() {
    println!("Hello, world!");
}
