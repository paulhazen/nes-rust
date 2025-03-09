#[derive(Debug, Clone)]
pub enum Mnemonic {

    ADC, AND, ASL, BCC, BCS, BEQ, BIT, BMI, BNE, BPL, BRK, BVC, BVS, CMP, CPX, 
    CPY, DEC, DEX, DEY, EOR, INC, INX, INY, JMP, JSR, LDA, LDX, LDY, LSR, NOP, 
    ORA, PHA, PHP, PLA, PLP, ROL, ROR, RTI, RTS, SBC, STA, STX, STY, TAX, TAY, 
    TSX, TXA, TXS, TYA, SEC, CLC, SED, CLD, SEI, CLI, CLV, 
}