use crate::state::State;
use crate::instruction::Instruction;

pub fn default(state: &mut State, instruction: &Instruction) -> bool {
    state.pc = instruction.get_addr() as usize;
    false
}

pub fn carry(state: &mut State, instruction: &Instruction) -> bool {
    if state.cc.cy == 1 {
        state.pc = instruction.get_addr() as usize;
    }
    false
}

pub fn no_carry(state: &mut State, instruction: &Instruction) -> bool {
    if state.cc.cy == 0 {
        state.pc = instruction.get_addr() as usize;
    }
    false
}

pub fn zero(state: &mut State, instruction: &Instruction) -> bool {
    if state.cc.z == 1 {
        state.pc = instruction.get_addr() as usize;
    }
    false
}

pub fn no_zero(state: &mut State, instruction: &Instruction) -> bool {
    if state.cc.z == 0 {
        state.pc = instruction.get_addr() as usize;
    }
    false
}

pub fn minus(state: &mut State, instruction: &Instruction) -> bool {
    if state.cc.s == 1 {
        state.pc = instruction.get_addr() as usize;
    }
    false
}

pub fn positive(state: &mut State, instruction: &Instruction) -> bool {
    if state.cc.s == 0 {
        state.pc = instruction.get_addr() as usize;
    }
    false
}

pub fn parity_even(state: &mut State, instruction: &Instruction) -> bool {
    if state.cc.p == 1 {
        state.pc = instruction.get_addr() as usize;
    }
    false
}

pub fn parity_odd(state: &mut State, instruction: &Instruction) -> bool {
    if state.cc.p == 0 {
        state.pc = instruction.get_addr() as usize;
    }
    false
}