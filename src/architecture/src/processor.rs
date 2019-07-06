use crate::opcodes::OpCode;
use crate::state::State;

pub fn determine_processing_unit(opcode: &OpCode) -> fn(&State) -> bool {
    match opcode {
        OpCode::NOP => no_operation,
        OpCode::LXI => load_register_pair,
        _ => not_implemented
    }
}

pub fn load_register_pair(state: &State) -> bool {
    println!("loading register pair");
    false
}

pub fn no_operation(state: &State) -> bool {
    println!("no operation....");
    false
}

pub fn not_implemented(state: &State) -> bool {
    println!("not implemented!");
    true
}