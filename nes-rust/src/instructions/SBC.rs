﻿use crate::cpu::CPU;
use crate::instruction::Instruction;
use crate::opcode::OpCode;

pub struct SBC;

#[inline(always)]
impl Instruction for SBC {
    fn execute(&self, cpu: &mut CPU, opcode: &OpCode) {
        // TODO: Implement execution logic for SBC
        println!("Executing SBC instruction");
    }
}
