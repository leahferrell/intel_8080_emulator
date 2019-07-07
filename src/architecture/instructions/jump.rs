use crate::architecture::state::State;
use crate::architecture::instruction::Instruction;

/// # Jump Instructions:
///
/// Instructions which alter the normal execution sequence of instructions.

pub fn default(state: &mut State, instruction: &Instruction) -> bool {
    state.pc = instruction.get_addr() as usize;
    false
}

fn condition(state: &mut State, instruction: &Instruction, expected_condition: bool) -> bool {
    if expected_condition {
        state.pc = instruction.get_addr() as usize;
    }else{
        state.pc += 1;
    }
    false
}

pub fn carry(state: &mut State, instruction: &Instruction) -> bool {
    condition(state, instruction, state.cc.cy)
}

pub fn no_carry(state: &mut State, instruction: &Instruction) -> bool {
    condition(state, instruction, !state.cc.cy)
}

pub fn zero(state: &mut State, instruction: &Instruction) -> bool {
    condition(state, instruction, state.cc.z)
}

pub fn no_zero(state: &mut State, instruction: &Instruction) -> bool {
    condition(state, instruction, !state.cc.z)
}

pub fn minus(state: &mut State, instruction: &Instruction) -> bool {
    condition(state, instruction, state.cc.s)
}

pub fn positive(state: &mut State, instruction: &Instruction) -> bool {
    condition(state, instruction, !state.cc.s)
}

pub fn parity_even(state: &mut State, instruction: &Instruction) -> bool {
    condition(state, instruction, state.cc.p)
}

pub fn parity_odd(state: &mut State, instruction: &Instruction) -> bool {
    condition(state, instruction, !state.cc.p)
}

pub fn load_pc(state: &mut State, instruction: &Instruction) -> bool {
    println!("ERROR: {} has not been implemented!", instruction.to_string());
    true
}