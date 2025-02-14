pub enum AddressingMode {

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
