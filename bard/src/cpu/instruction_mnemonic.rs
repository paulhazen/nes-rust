#[derive(Debug, Clone)]
pub enum InstructionMnemonic {

    ADC, AND, ASL, BCC, BCS, BEQ, BIT, BMI, BNE, BPL, BRK, BVC, BVS, CMP, CPX, 
    CPY, DEC, DEX, DEY, EOR, INC, INX, INY, JMP, JSR, LDA, LDX, LDY, LSR, NOP, 
    ORA, PHA, PHP, PLA, PLP, ROL, ROR, RTI, RTS, SBC, STA, STX, STY, TAX, TAY, 
    TSX, TXA, TXS, TYA,

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