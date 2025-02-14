use crate::cpu::CPU;
use crate::instruction::Instruction;
use crate::opcode::OpCode;

pub struct BVS;

#[inline(always)]
impl Instruction for BVS {
    fn execute(&self, cpu: &mut CPU, opcode: &OpCode) {
        // TODO: Implement execution logic for BVS
        println!("Executing BVS instruction");
    }
}
