use crate::cpu::status_register::StatusRegister;
use crate::cpu::instruction::Instruction;
use crate::cpu::opcode::OpCodeExecutor;
use crate::cpu::opcode::OpCode;
use crate::opcode_entry;
use crate::memory::MemoryBus;
use std::collections::HashMap;
use once_cell::sync::Lazy;

static OPCODE_TABLE : Lazy<HashMap<u8, OpCode>> = Lazy::new(|| {
    let mut map = HashMap::new();

    // region: Opcodes

    /*
        ADC - Add with Carry

        A,Z,C,N = A+M+C
        This instruction adds the contents of a memory location to the accumulator 
        together with the carry bit. If overflow occurs the carry bit is set, this 
        enables multiple byte addition to be performed.
    */
    //opcode_entry!(map,     0x69,      ADC,      2,      2,       Immediate   ); 
    //opcode_entry!(map,     0x65,      ADC,      2,      3,       ZeroPage    ); 
    //opcode_entry!(map,     0x75,      ADC,      2,      4,       ZeroPageX   ); 
    //opcode_entry!(map,     0x6D,      ADC,      3,      4,       Absolute    ); 
    //opcode_entry!(map,     0x7D,      ADC,      3,      4,       AbsoluteX   ); // +1 if page crossed
    //opcode_entry!(map,     0x79,      ADC,      3,      4,       AbsoluteY   ); // +1 if page crossed
    //opcode_entry!(map,     0x61,      ADC,      2,      6,       IndirectX   ); 
    //opcode_entry!(map,     0x71,      ADC,      2,      5,       IndirectY   ); // +1 if page crossed
//
    ///*
    //    AND - Logical AND
//
    //    A,Z,N = A&M
    //    A logical AND is performed, bit by bit, on the accumulator contents
    //        using the contents of a byte of memory.
    //*/
    //opcode_entry!(map,     0x29,      AND,      2,      2,       Immediate   ); 
    //opcode_entry!(map,     0x25,      AND,      2,      3,       ZeroPage    ); 
    //opcode_entry!(map,     0x35,      AND,      2,      4,       ZeroPageX   ); 
    //opcode_entry!(map,     0x2D,      AND,      3,      4,       Absolute    ); 
    //opcode_entry!(map,     0x3D,      AND,      3,      4,       AbsoluteX   ); // +1 if page crossed
    //opcode_entry!(map,     0x39,      AND,      3,      4,       AbsoluteY   ); // +1 if page crossed
    //opcode_entry!(map,     0x21,      AND,      2,      6,       IndirectX   ); 
    //opcode_entry!(map,     0x31,      AND,      2,      5,       IndirectY   ); // +1 if page crossed
//
    ///*
    //    ASL - Arithmetic Shift Left
//
    //    A,Z,C,N = M*2 or M,Z,C,N = M*2
    //    This operation shifts all the bits of the accumulator or memory 
    //    contents one bit left. Bit 0 is set to 0 and bit 7 is placed in the
    //    carry flag. The effect of this operation is to multiply the memory
    //    contents by 2 (ignoring 2's complement considerations), setting
    //    the carry if the result will not fit in 8 bits.
    //*/
    //opcode_entry!(map,     0x0A,      ASL,      1,      2,       Accumulator ); 
    //opcode_entry!(map,     0x06,      ASL,      2,      5,       ZeroPage    ); 
    //opcode_entry!(map,     0x16,      ASL,      2,      6,       ZeroPageX   ); 
    //opcode_entry!(map,     0x0E,      ASL,      3,      6,       Absolute    ); 
    //opcode_entry!(map,     0x1E,      ASL,      3,      7,       AbsoluteX   ); 
//
    ///*
    //    BCC - Branch if Carry Clear
//
    //    If the carry flag is clear then add the relative displacement to the program counter to cause a branch to a new location.
    //*/
    //opcode_entry!(map,     0x90,      BCC,      2,      2,       Relative    ); // +1 cycles if branch succeeds +2 if to a new page
//
    ///*
    //    BCS - Branch if Carry Set
//
    //    If the carry flag is set then add the relative displacement to the program counter to cause a branch to a new location.
    //*/
    //opcode_entry!(map,     0xB0,      BCS,      2,      2,       Relative    ); // +1 cycles if branch succeeds +2 if to a new page
//
    ///*
    //    BEQ - Branch if Equal
//
    //    If the zero flag is set then add the relative displacement to the program counter to cause a branch to a new location.
    //*/
    //opcode_entry!(map,     0xF0,      BEQ,      2,      2,       Relative    ); // +1 cycles if branch succeeds +2 if to a new page
//
    ///*
    //    BIT - Bit Test
//
    //    A & M, N = M7, V = M6
    //    This instructions is used to test if one or more bits are set in a target memory location. The mask pattern in A is ANDed with the value in memory to set or clear the zero flag, but the result is not kept. Bits 7 and 6 of the value from memory are copied into the N and V flags.
    //*/
    //opcode_entry!(map,     0x24,      BIT,      2,      3,       ZeroPage    ); 
    //opcode_entry!(map,     0x2C,      BIT,      3,      4,       Absolute    ); 
//
    ///*
    //    BMI - Branch if Minus
//
    //    If the negative flag is set then add the relative displacement to the program counter to cause a branch to a new location.
    //*/
    //opcode_entry!(map,     0x30,      BMI,      2,      2,       Relative    ); // +1 cycles if branch succeeds +2 if to a new page
//
    ///*
    //    BNE - Branch if Not Equal
//
    //    If the zero flag is clear then add the relative displacement to the program counter to cause a branch to a new location.
    //*/
    //opcode_entry!(map,     0xD0,      BNE,      2,      2,       Relative    ); // +1 cycles if branch succeeds +2 if to a new page
//
    ///*
    //    BPL - Branch if Positive
//
    //    If the negative flag is clear then add the relative displacement to the program counter to cause a branch to a new location.
    //*/
    //opcode_entry!(map,     0x10,      BPL,      2,      2,       Relative    ); // +1 cycles if branch succeeds +2 if to a new page
//
    ///*
    //    BRK - Force Interrupt
//
    //    The BRK instruction forces the generation of an interrupt request. The program counter and processor status are pushed on the stack then the IRQ interrupt vector at $FFFE/F is loaded into the PC and the break flag in the status set to one.
    //*/
    //opcode_entry!(map,     0x00,      BRK,      1,      7,       Implied     ); 
//
    ///*
    //    BVC - Branch if Overflow Clear
//
    //    If the overflow flag is clear then add the relative displacement to the program counter to cause a branch to a new location.
    //*/
    //opcode_entry!(map,     0x50,      BVC,      2,      2,       Relative    ); // +1 cycles if branch succeeds +2 if to a new page
//
    ///*
    //    BVS - Branch if Overflow Set
//
    //    If the overflow flag is set then add the relative displacement to the program counter to cause a branch to a new location.
    //*/
    //opcode_entry!(map,     0x70,      BVS,      2,      2,       Relative    ); // +1 cycles if branch succeeds +2 if to a new page
//
    ///*
    //    CLC - Clear Carry Flag
//
    //    C = 0
    //    Set the carry flag to zero.
    //*/
    //opcode_entry!(map,     0x18,      CLC,      1,      2,       Implied     ); 
//
    ///*
    //    CLD - Clear Decimal Mode
//
    //    D = 0
    //    Sets the decimal mode flag to zero.
    //*/
    //opcode_entry!(map,     0xD8,      CLD,      1,      2,       Implied     ); 
//
    ///*
    //    CLI - Clear Interrupt Disable
//
    //    I = 0
    //    Clears the interrupt disable flag allowing normal interrupt requests to be serviced.
    //*/
    //opcode_entry!(map,     0x58,      CLI,      1,      2,       Implied     ); 
//
    ///*
    //    CLV - Clear Overflow Flag
//
    //    V = 0
    //    Clears the overflow flag.
    //*/
    //opcode_entry!(map,     0xB8,      CLV,      1,      2,       Implied     ); 
//
    ///*
    //    CMP - Compare
//
    //    Z,C,N = A-M
    //    This instruction compares the contents of the accumulator with another memory held value and sets the zero and carry flags as appropriate.
    //*/
    //opcode_entry!(map,     0xC9,      CMP,      2,      2,       Immediate   ); 
    //opcode_entry!(map,     0xC5,      CMP,      2,      3,       ZeroPage    ); 
    //opcode_entry!(map,     0xD5,      CMP,      2,      4,       ZeroPageX   ); 
    //opcode_entry!(map,     0xCD,      CMP,      3,      4,       Absolute    ); 
    //opcode_entry!(map,     0xDD,      CMP,      3,      4,       AbsoluteX   ); // +1 if page crossed
    //opcode_entry!(map,     0xD9,      CMP,      3,      4,       AbsoluteY   ); // +1 if page crossed
    //opcode_entry!(map,     0xC1,      CMP,      2,      6,       IndirectX   ); 
    //opcode_entry!(map,     0xD1,      CMP,      2,      5,       IndirectY   ); // +1 if page crossed
//
    ///*
    //    CPX - Compare X Register
//
    //    Z,C,N = X-M
    //    This instruction compares the contents of the X register with another memory held value and sets the zero and carry flags as appropriate.
    //*/
    //opcode_entry!(map,     0xE0,      CPX,      2,      2,       Immediate   ); 
    //opcode_entry!(map,     0xE4,      CPX,      2,      3,       ZeroPage    ); 
    //opcode_entry!(map,     0xEC,      CPX,      3,      4,       Absolute    ); 
//
    ///*
    //    CPY - Compare Y Register
//
    //    Z,C,N = Y-M
    //    This instruction compares the contents of the Y register with another memory held value and sets the zero and carry flags as appropriate.
    //*/
    //opcode_entry!(map,     0xC0,      CPY,      2,      2,       Immediate   ); 
    //opcode_entry!(map,     0xC4,      CPY,      2,      3,       ZeroPage    ); 
    //opcode_entry!(map,     0xCC,      CPY,      3,      4,       Absolute    ); 
//
    ///*
    //    DEC - Decrement Memory
//
    //    M,Z,N = M-1
    //    Subtracts one from the value held at a specified memory location setting the zero and negative flags as appropriate.
    //*/
    //opcode_entry!(map,     0xC6,      DEC,      2,      5,       ZeroPage    ); 
    //opcode_entry!(map,     0xD6,      DEC,      2,      6,       ZeroPageX   ); 
    //opcode_entry!(map,     0xCE,      DEC,      3,      6,       Absolute    ); 
    //opcode_entry!(map,     0xDE,      DEC,      3,      7,       AbsoluteX   ); 
//
    ///*
    //    DEX - Decrement X Register
//
    //    X,Z,N = X-1
    //    Subtracts one from the X register setting the zero and negative flags as appropriate.
    //*/
    //opcode_entry!(map,     0xCA,      DEX,      1,      2,       Implied     ); 
//
    ///*
    //    DEY - Decrement Y Register
//
    //    Y,Z,N = Y-1
    //    Subtracts one from the Y register setting the zero and negative flags as appropriate.
    //*/
    //opcode_entry!(map,     0x88,      DEY,      1,      2,       Implied     ); 
//
    ///*
    //    EOR - Exclusive OR
//
    //    A,Z,N = A^M
    //    An exclusive OR is performed, bit by bit, on the accumulator contents using the contents of a byte of memory.
    //*/
    //opcode_entry!(map,     0x49,      EOR,      2,      2,       Immediate   ); 
    //opcode_entry!(map,     0x45,      EOR,      2,      3,       ZeroPage    ); 
    //opcode_entry!(map,     0x55,      EOR,      2,      4,       ZeroPageX   ); 
    //opcode_entry!(map,     0x4D,      EOR,      3,      4,       Absolute    ); 
    //opcode_entry!(map,     0x5D,      EOR,      3,      4,       AbsoluteX   ); // +1 if page crossed
    //opcode_entry!(map,     0x59,      EOR,      3,      4,       AbsoluteY   ); // +1 if page crossed
    //opcode_entry!(map,     0x41,      EOR,      2,      6,       IndirectX   ); 
    //opcode_entry!(map,     0x51,      EOR,      2,      5,       IndirectY   ); // +1 if page crossed
//
    ///*
    //    INC - Increment Memory
//
    //    M,Z,N = M+1
    //    Adds one to the value held at a specified memory location setting the zero and negative flags as appropriate.
    //*/
    //opcode_entry!(map,     0xE6,      INC,      2,      5,       ZeroPage    ); 
    //opcode_entry!(map,     0xF6,      INC,      2,      6,       ZeroPageX   ); 
    //opcode_entry!(map,     0xEE,      INC,      3,      6,       Absolute    ); 
    //opcode_entry!(map,     0xFE,      INC,      3,      7,       AbsoluteX   ); 
//
    ///*
    //    INX - Increment X Register
//
    //    X,Z,N = X+1
    //    Adds one to the X register setting the zero and negative flags as appropriate.
    //*/
    //opcode_entry!(map,     0xE8,      INX,      1,      2,       Implied     ); 
//
    ///*
    //    INY - Increment Y Register
//
    //    Y,Z,N = Y+1
    //    Adds one to the Y register setting the zero and negative flags as appropriate.
    //*/
    //opcode_entry!(map,     0xC8,      INY,      1,      2,       Implied     ); 
//
    ///*
    //    JMP - Jump
//
    //    Sets the program counter to the address specified by the operand.
    //*/
    //opcode_entry!(map,     0x4C,      JMP,      3,      3,       Absolute    ); 
    //opcode_entry!(map,     0x6C,      JMP,      3,      5,       Indirect    ); 
//
    ///*
    //    JSR - Jump to Subroutine
//
    //    The JSR instruction pushes the address (minus one) of the return point on to the stack and then sets the program counter to the target memory address.
    //*/
    //opcode_entry!(map,     0x20,      JSR,      3,      6,       Absolute    ); 

    /*
        LDA - Load Accumulator

        A,Z,N = M
        Loads a byte of memory into the accumulator setting the zero and negative flags as appropriate.
    */
    opcode_entry!(map,     0xA9,      LDA,      2,      2,       Immediate   ); 
    opcode_entry!(map,     0xA5,      LDA,      2,      3,       ZeroPage    ); 
    opcode_entry!(map,     0xB5,      LDA,      2,      4,       ZeroPageX   ); 
    opcode_entry!(map,     0xAD,      LDA,      3,      4,       Absolute    ); 
    opcode_entry!(map,     0xBD,      LDA,      3,      4,       AbsoluteX   ); // +1 if page crossed
    opcode_entry!(map,     0xB9,      LDA,      3,      4,       AbsoluteY   ); // +1 if page crossed
    opcode_entry!(map,     0xA1,      LDA,      2,      6,       IndirectX   ); 
    opcode_entry!(map,     0xB1,      LDA,      2,      5,       IndirectY   ); // +1 if page crossed

    /*
        LDX - Load X Register

        X,Z,N = M
        Loads a byte of memory into the X register setting the zero and negative flags as appropriate.
    */
    //opcode_entry!(map,     0xA2,      LDX,      2,      2,       Immediate   ); 
    //opcode_entry!(map,     0xA6,      LDX,      2,      3,       ZeroPage    ); 
    //opcode_entry!(map,     0xB6,      LDX,      2,      4,       ZeroPageY   ); 
    //opcode_entry!(map,     0xAE,      LDX,      3,      4,       Absolute    ); 
    //opcode_entry!(map,     0xBE,      LDX,      3,      4,       AbsoluteY   ); // +1 if page crossed
//
    ///*
    //    LDY - Load Y Register
//
    //    Y,Z,N = M
    //    Loads a byte of memory into the Y register setting the zero and negative flags as appropriate.
    //*/
    //opcode_entry!(map,     0xA0,      LDY,      2,      2,       Immediate   ); 
    //opcode_entry!(map,     0xA4,      LDY,      2,      3,       ZeroPage    ); 
    //opcode_entry!(map,     0xB4,      LDY,      2,      4,       ZeroPageX   ); 
    //opcode_entry!(map,     0xAC,      LDY,      3,      4,       Absolute    ); 
    //opcode_entry!(map,     0xBC,      LDY,      3,      4,       AbsoluteX   ); // +1 if page crossed
//
    ///*
    //    LSR - Logical Shift Right
//
    //    A,C,Z,N = A/2 or M,C,Z,N = M/2
    //    Each of the bits in A or M is shift one place to the right. The bit that was in bit 0 is shifted into the carry flag. Bit 7 is set to zero.
    //*/
    //opcode_entry!(map,     0x4A,      LSR,      1,      2,       Accumulator ); 
    //opcode_entry!(map,     0x46,      LSR,      2,      5,       ZeroPage    ); 
    //opcode_entry!(map,     0x56,      LSR,      2,      6,       ZeroPageX   ); 
    //opcode_entry!(map,     0x4E,      LSR,      3,      6,       Absolute    ); 
    //opcode_entry!(map,     0x5E,      LSR,      3,      7,       AbsoluteX   ); 
//
    ///*
    //    NOP - No Operation
//
    //    The NOP instruction causes no changes to the processor other than the normal incrementing of the program counter to the next instruction.
    //*/
    //opcode_entry!(map,     0xEA,      NOP,      1,      2,       Implied     ); 
//
    ///*
    //    ORA - Logical Inclusive OR
//
    //    A,Z,N = A|M
    //    An inclusive OR is performed, bit by bit, on the accumulator contents using the contents of a byte of memory.
    //*/
    //opcode_entry!(map,     0x09,      ORA,      2,      2,       Immediate   ); 
    //opcode_entry!(map,     0x05,      ORA,      2,      3,       ZeroPage    ); 
    //opcode_entry!(map,     0x15,      ORA,      2,      4,       ZeroPageX   ); 
    //opcode_entry!(map,     0x0D,      ORA,      3,      4,       Absolute    ); 
    //opcode_entry!(map,     0x1D,      ORA,      3,      4,       AbsoluteX   ); // +1 if page crossed
    //opcode_entry!(map,     0x19,      ORA,      3,      4,       AbsoluteY   ); // +1 if page crossed
    //opcode_entry!(map,     0x01,      ORA,      2,      6,       IndirectX   ); 
    //opcode_entry!(map,     0x11,      ORA,      2,      5,       IndirectY   ); // +1 if page crossed
//
    ///*
    //    PHA - Push Accumulator
//
    //    Pushes a copy of the accumulator on to the stack.
    //*/
    //opcode_entry!(map,     0x48,      PHA,      1,      3,       Implied     ); 
//
    ///*
    //    PHP - Push Processor Status
//
    //    Pushes a copy of the status flags on to the stack.
    //*/
    //opcode_entry!(map,     0x08,      PHP,      1,      3,       Implied     ); 
//
    ///*
    //    PLA - Pull Accumulator
//
    //    Pulls an 8 bit value from the stack and into the accumulator. The zero and negative flags are set as appropriate.
    //*/
    //opcode_entry!(map,     0x68,      PLA,      1,      4,       Implied     ); 
//
    ///*
    //    PLP - Pull Processor Status
//
    //    Pulls an 8 bit value from the stack and into the processor flags. The flags will take on new states as determined by the value pulled.
    //*/
    //opcode_entry!(map,     0x28,      PLP,      1,      4,       Implied     ); 
//
    ///*
    //    ROL - Rotate Left
//
    //    Move each of the bits in either A or M one place to the left. Bit 0 is filled with the current value of the carry flag whilst the old bit 7 becomes the new carry flag value.
    //*/
    //opcode_entry!(map,     0x2A,      ROL,      1,      2,       Accumulator ); 
    //opcode_entry!(map,     0x26,      ROL,      2,      5,       ZeroPage    ); 
    //opcode_entry!(map,     0x36,      ROL,      2,      6,       ZeroPageX   ); 
    //opcode_entry!(map,     0x2E,      ROL,      3,      6,       Absolute    ); 
    //opcode_entry!(map,     0x3E,      ROL,      3,      7,       AbsoluteX   ); 
//
    ///*
    //    ROR - Rotate Right
//
    //    Move each of the bits in either A or M one place to the right. Bit 7 is filled with the current value of the carry flag whilst the old bit 0 becomes the new carry flag value.
    //*/
    //opcode_entry!(map,     0x6A,      ROR,      1,      2,       Accumulator ); 
    //opcode_entry!(map,     0x66,      ROR,      2,      5,       ZeroPage    ); 
    //opcode_entry!(map,     0x76,      ROR,      2,      6,       ZeroPageX   ); 
    //opcode_entry!(map,     0x6E,      ROR,      3,      6,       Absolute    ); 
    //opcode_entry!(map,     0x7E,      ROR,      3,      7,       AbsoluteX   ); 
//
    ///*
    //    RTI - Return from Interrupt
//
    //    The RTI instruction is used at the end of an interrupt processing routine. It pulls the processor flags from the stack followed by the program counter.
    //*/
    //opcode_entry!(map,     0x40,      RTI,      1,      6,       Implied     ); 
//
    ///*
    //    RTS - Return from Subroutine
//
    //    The RTS instruction is used at the end of a subroutine to return to the calling routine. It pulls the program counter (minus one) from the stack.
    //*/
    //opcode_entry!(map,     0x60,      RTS,      1,      6,       Implied     ); 
//
    ///*
    //    SBC - Subtract with Carry
//
    //    A,Z,C,N = A-M-(1-C)
    //    This instruction subtracts the contents of a memory location to the accumulator together with the not of the carry bit. If overflow occurs the carry bit is clear, this enables multiple byte subtraction to be performed.
    //*/
    //opcode_entry!(map,     0xE9,      SBC,      2,      2,       Immediate   ); 
    //opcode_entry!(map,     0xE5,      SBC,      2,      3,       ZeroPage    ); 
    //opcode_entry!(map,     0xF5,      SBC,      2,      4,       ZeroPageX   ); 
    //opcode_entry!(map,     0xED,      SBC,      3,      4,       Absolute    ); 
    //opcode_entry!(map,     0xFD,      SBC,      3,      4,       AbsoluteX   ); // +1 if page crossed
    //opcode_entry!(map,     0xF9,      SBC,      3,      4,       AbsoluteY   ); // +1 if page crossed
    //opcode_entry!(map,     0xE1,      SBC,      2,      6,       IndirectX   ); 
    //opcode_entry!(map,     0xF1,      SBC,      2,      5,       IndirectY   ); // +1 if page crossed
//
    ///*
    //    SEC - Set Carry Flag
//
    //    C = 1
    //    Set the carry flag to one.
    //*/
    //opcode_entry!(map,     0x38,      SEC,      1,      2,       Implied     ); 
//
    ///*
    //    SED - Set Decimal Flag
//
    //    D = 1
    //    Set the decimal mode flag to one.
    //*/
    //opcode_entry!(map,     0xF8,      SED,      1,      2,       Implied     ); 
//
    ///*
    //    SEI - Set Interrupt Disable
//
    //    I = 1
    //    Set the interrupt disable flag to one.
    //*/
    //opcode_entry!(map,     0x78,      SEI,      1,      2,       Implied     ); 
//
    ///*
    //    STA - Store Accumulator
//
    //    M = A
    //    Stores the contents of the accumulator into memory.
    //*/
    //opcode_entry!(map,     0x85,      STA,      2,      3,       ZeroPage    ); 
    //opcode_entry!(map,     0x95,      STA,      2,      4,       ZeroPageX   ); 
    //opcode_entry!(map,     0x8D,      STA,      3,      4,       Absolute    ); 
    //opcode_entry!(map,     0x9D,      STA,      3,      5,       AbsoluteX   ); 
    //opcode_entry!(map,     0x99,      STA,      3,      5,       AbsoluteY   ); 
    //opcode_entry!(map,     0x81,      STA,      2,      6,       IndirectX   ); 
    //opcode_entry!(map,     0x91,      STA,      2,      6,       IndirectY   ); 
//
    ///*
    //    STX - Store X Register
//
    //    M = X
    //    Stores the contents of the X register into memory.
    //*/
    //opcode_entry!(map,     0x86,      STX,      2,      3,       ZeroPage    ); 
    //opcode_entry!(map,     0x96,      STX,      2,      4,       ZeroPageY   ); 
    //opcode_entry!(map,     0x8E,      STX,      3,      4,       Absolute    ); 
//
    ///*
    //    STY - Store Y Register
//
    //    M = Y
    //    Stores the contents of the Y register into memory.
    //*/
    //opcode_entry!(map,     0x84,      STY,      2,      3,       ZeroPage    ); 
    //opcode_entry!(map,     0x94,      STY,      2,      4,       ZeroPageX   ); 
    //opcode_entry!(map,     0x8C,      STY,      3,      4,       Absolute    ); 
//
    ///*
    //    TAX - Transfer Accumulator to X
//
    //    X = A
    //    Copies the current contents of the accumulator into the X register and sets the zero and negative flags as appropriate.
    //*/
    //opcode_entry!(map,     0xAA,      TAX,      1,      2,       Implied     ); 
//
    ///*
    //    TAY - Transfer Accumulator to Y
//
    //    Y = A
    //    Copies the current contents of the accumulator into the Y register and sets the zero and negative flags as appropriate.
    //*/
    //opcode_entry!(map,     0xA8,      TAY,      1,      2,       Implied     ); 
//
    ///*
    //    TSX - Transfer Stack Pointer to X
//
    //    X = S
    //    Copies the current contents of the stack register into the X register and sets the zero and negative flags as appropriate.
    //*/
    //opcode_entry!(map,     0xBA,      TSX,      1,      2,       Implied     ); 
//
    ///*
    //    TXA - Transfer X to Accumulator
//
    //    A = X
    //    Copies the current contents of the X register into the accumulator and sets the zero and negative flags as appropriate.
    //*/
    //opcode_entry!(map,     0x8A,      TXA,      1,      2,       Implied     ); 
//
    ///*
    //    TXS - Transfer X to Stack Pointer
//
    //    S = X
    //    Copies the current contents of the X register into the stack register.
    //*/
    //opcode_entry!(map,     0x9A,      TXS,      1,      2,       Implied     ); 
//
    ///*
    //    TYA - Transfer Y to Accumulator
//
    //    A = Y
    //    Copies the current contents of the Y register into the accumulator and sets the zero and negative flags as appropriate.
    //*/
    //opcode_entry!(map,     0x98,      TYA,      1,      2,       Implied     ); 

    // endregion: Opcodes

    map
});

pub struct CPU {
    accumulator: u8,
    x_register: u8,
    y_register: u8,
    program_counter: u16,             
    stack_pointer: u8,
    pub processor_status: StatusRegister,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            accumulator: 0, 
            x_register: 0,
            y_register: 0, 
            program_counter: 0x8000, // NES program entry point
            stack_pointer: 0xFD,
            processor_status: StatusRegister::new(),
        }
    }

    pub fn step(&mut self, memory: &mut MemoryBus) {
        let pc = self.program_counter;
        let opcode = self.fetch_byte(memory);
        self.execute(opcode, memory);
    }

    // region: Setter / Getter methods

    pub fn get_program_counter(&self) -> u16 {
        self.program_counter
    }

    pub fn set_accumulator(&mut self, value: u8) {
        self.accumulator = value;
    }

    pub fn get_x_register(&self) -> u8 {
        self.x_register
    }

    pub fn get_y_register(&self) -> u8 {
        self.y_register
    }

    // endregion: Accessor methods

    pub fn execute(&mut self, opcode: u8, memory: &mut MemoryBus) {
        println!("Looking up opcode {0}", opcode);
        if let Some(instruction) = OPCODE_TABLE.get(&opcode) {
            println!("Found opcode for instruction");
            (instruction.factory)();
        } else {
            println!("Could not find instruction for opcode.");
        }
    }

    pub fn show_opcode_table(&self) {
        for (key, value) in OPCODE_TABLE.iter() {
            println!("{0:>3}   {1:#?}   ({3} bytes {4} cycles); Mode: {2:#?}", key, value.mnemonic, value.mode, value.size, value.cycles);
        }        
    }

    pub fn update_zero_and_negative_flags(&mut self, value: u8) {
        // Update Zero flag (Z): Set if value is 0
        if value == 0 {
            self.processor_status.set(StatusRegister::ZERO);
        }

        // Update Negative flag (N): Set if the highest bit (bit 7) is 1
        if value & 0b1000_0000 != 0 {
            self.processor_status.set(StatusRegister::NEGATIVE);
        }
    }

    /**
     * Executes the given op code executor.
     */
    #[inline(always)]
    pub fn execute_instruction<T: Instruction>(&mut self, executor: OpCodeExecutor<T>, memory: &mut MemoryBus) {
        executor.execute(self, memory);
    }

    pub fn reset(&mut self, memory: &MemoryBus) {
        let low_byte = memory.read(MemoryBus::RESET_VECTOR_ADDR) as u16;
        let high_byte = memory.read(MemoryBus::RESET_VECTOR_HIGH_ADDR) as u16;

        self.program_counter = (high_byte << 8) | low_byte;

        // Reset the processor status
        self.processor_status.reset()
    }

    // startregion: Fetch functions

    pub fn fetch_immediate(&mut self, memory: &MemoryBus) -> u8 {
        self.fetch_byte(memory)
    }

    pub fn fetch_zero_page(&mut self, memory: &MemoryBus) -> u8 {
        let address = self.fetch_byte(memory) as u16;
        memory.read(address)
    }

    pub fn fetch_zero_page_x(&mut self, memory: &MemoryBus) -> u8 {
        let address = self.fetch_byte(memory).wrapping_add(self.get_x_register()) as u16;
        memory.read(address)
    }

    pub fn fetch_zero_page_y(&mut self, memory: &MemoryBus) -> u8 {
        let address = self.fetch_byte(memory).wrapping_add(self.y_register) as u16;
        memory.read(address)
    }

    pub fn fetch_absolute(&mut self, memory: &MemoryBus) -> u8 {
        let low = self.fetch_byte(memory) as u16;
        let high = self.fetch_byte(memory) as u16;
        let address = (high << 8) | low;
        memory.read(address)
    }

    pub fn fetch_absolute_x(&mut self, memory: &MemoryBus) -> u8 {
        let low = self.fetch_byte(memory) as u16;
        let high = self.fetch_byte(memory) as u16;
        let base_address = (high << 8) | low;
        let address = base_address.wrapping_add(self.get_x_register() as u16);
        memory.read(address)
    }

    pub fn fetch_absolute_y(&mut self, memory: &MemoryBus) -> u8 {
        let low = self.fetch_byte(memory) as u16;
        let high = self.fetch_byte(memory) as u16;
        let base_address = (high << 8) | low;
        let address = base_address.wrapping_add(self.get_y_register() as u16);
        memory.read(address)
    }

    pub fn fetch_indirect_x(&mut self, memory: &MemoryBus) -> u8 {
        let base_address = self.fetch_byte(memory);
        let zero_page_address = base_address.wrapping_add(self.get_x_register());

        let low = memory.read(zero_page_address as u16) as u16;
        let high = memory.read(zero_page_address.wrapping_add(1) as u16) as u16;
        let effective_address = (high << 8) | low;

        memory.read(effective_address)
    }

    pub fn fetch_indirect_y(&mut self, memory: &MemoryBus) -> u8 {
        let base_address = self.fetch_byte(memory);

        let low = memory.read(base_address as u16) as u16;
        let high = memory.read(base_address.wrapping_add(1) as u16) as u16;
        let base_address = (high << 8) | low;
        let effective_address = base_address.wrapping_add(self.get_y_register() as u16);

        memory.read(effective_address)
    }

    pub fn fetch_byte(&mut self, memory: &MemoryBus) -> u8 {
        let opcode = memory.read(self.program_counter);
        self.program_counter = self.program_counter.wrapping_add(1);
        opcode
    }

    // endregion: Fetch functions.
}