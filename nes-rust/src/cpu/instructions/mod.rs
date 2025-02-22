// Declare each instruction module

pub mod and;
pub mod asl;
pub mod bcc;
pub mod bcs;
pub mod beq;
pub mod bit;
pub mod bmi;
pub mod bne;
pub mod bpl;
pub mod brk;
pub mod bvc;
pub mod bvs;
pub mod dec;
pub mod dex;
pub mod dey;
pub mod eor;
pub mod inc;
pub mod inx;
pub mod iny;
pub mod jmp;
pub mod jsr;
pub mod lsr;
pub mod nop;
pub mod ora;
pub mod pha;
pub mod php;
pub mod pla;
pub mod rol;
pub mod ror;
pub mod rti;
pub mod rts;
pub mod tax;
pub mod tay;
pub mod tsx;
pub mod txa;
pub mod txs;
pub mod tya;
pub mod flags;
pub mod load_store;
pub mod arithmetic;
//// Re-export each so they can be accessed from `instruction::`
pub use and::AND;
//pub use asl::ASL;
//pub use bcc::BCC;
//pub use bcs::BCS;
pub use beq::BEQ;
pub use bit::BIT;
//pub use bmi::BMI;
//pub use bne::BNE;
pub use bpl::BPL;
//pub use brk::BRK;
//pub use bvc::BVC;
//pub use bvs::BVS;
//pub use dec::DEC;
pub use dex::DEX;
pub use dey::DEY;
pub use eor::EOR;
//pub use inc::INC;
pub use inx::INX;
pub use iny::INY;
//pub use jmp::JMP;
pub use jsr::JSR;
//pub use lsr::LSR;
pub use nop::NOP;
pub use ora::ORA;
//pub use rol::ROL;
//pub use ror::ROR;
pub use rts::RTS;
//pub use sbc::SBC;
pub use tax::TAX;
pub use tay::TAY;
pub use tsx::TSX;
pub use txa::TXA;
pub use txs::TXS;
pub use tya::TYA;

pub use arithmetic::{ADC, CMP, /* CPX, CPY, SBC */};
pub use flags::{CLC, CLD, CLI, CLV, SEI};
pub use load_store::{LDA, LDX, LDY, STA, STX, STY};