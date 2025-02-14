struct CPU {
    a: u8,   // Accumulator
    x: u8,   // X Register
    y: u8,   // Y Register
    pc: u16, // Program Counter
    s: u8,   // Stack Pointer
    p: u8,   // Processor Status
}

enum OpCode {
    ADC, // Add with carry A,Z,C,N = A + M + C
    AND, // Logical AND (A,Z,N = A&M)
    ASL, // Arithmetic Shift Left (A,Z,C,N = M*2 or M,Z,C,N = M*2)
    BCC, //
    BCS,
    BEQ,
    BIT,
    BMI, 
    BNE,
    BPL,
    BRK,
    BVC,
    BVS,
    CLC,
    CLD,
    CLI,
    CLV,
    CMP,
    CPX,
    CPY,
    DEC,
    DEX,
    DEY,
    EOR,
    INC,
    INX,
    INY,
    JMP,
    JSR,
    LDA,
    LDX,
    LDY,
    LSR,
    NOP,
    ORA,
    PHA,
    PHP,
    PLA,
    ROL,
    ROR,
    RTI,
    RTS,
    SBC,
    SEC,
    SED,
    SEI,
    STA,
    STX,
    STY,
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA,
}

impl CPU {
    fn new() -> Self {
        CPU {
            a: 0, 
            x: 0,
            y: 0, 
            pc: 0x8000, // NES program entry point
            s: 0xFD,    // Default stack pointer
            p: 0x24,    // Default status flag
        }
    }
}

fn main() {
    println!("Hello, world!");
}
