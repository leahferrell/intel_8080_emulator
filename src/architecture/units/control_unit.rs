use crate::architecture::state::State;
use crate::architecture::instruction::Instruction;
use crate::architecture::units::stack;
use crate::architecture::opcodes::OpCode::*;

pub fn default(state: &mut State, instruction: &Instruction) -> bool {

    match instruction.opcode.to_string().chars().next() {
        Some('C') => {
            let ret:u16 = (state.pc + instruction.num_of_bytes()) as u16;
            println!("Setting return address: {:04x}", ret);
            stack::push_return_addr(state, ret);
            state.pc = instruction.get_addr() as usize;
        },
        Some('J') => {
            state.pc = instruction.get_addr() as usize;
        },
        Some('R') => {
            let ret:u16 = stack::pop_return_addr(state);
            println!("Getting return address: {:04x}", ret);
            state.pc = ret as usize;
        },
        _ => ()
    };
    false
}

fn condition(state: &mut State, instruction: &Instruction, expected_condition: bool) -> bool {
    if expected_condition {
        default(state, instruction);
    }else{
        state.pc += instruction.num_of_bytes();
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