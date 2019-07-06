use crate::opcodes::OpCode;
use crate::state::State;
use crate::instructions::*;
use crate::instruction::Instruction;

pub fn determine_processing_unit(opcode: &OpCode) -> fn(&State, &Instruction) -> bool {
    match opcode {
        OpCode::NOP => nop::no_operation,
        OpCode::LXI => immediate::load_register_pair,
        _ => not_implemented
    }
}