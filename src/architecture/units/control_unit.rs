use crate::architecture::model::state::State;
use crate::architecture::model::instruction::Instruction;
use crate::architecture::units::stack;
use crate::architecture::units::arithmetic_logic_unit;
use crate::architecture::AddressPtr;

pub fn default(state: &mut State, instruction: &Instruction) -> AddressPtr {

    match instruction.opcode.to_string().chars().next() {
        Some('C') => {
            let ret:u16 = (state.pc + instruction.size()) as u16;
            println!("Setting return address: {:04x}", ret);
            stack::push_return_addr(state, ret);
            arithmetic_logic_unit::u8_to_u16(instruction.operands[0], instruction.operands[1]) as usize
        },
        Some('J') => {
            arithmetic_logic_unit::u8_to_u16(instruction.operands[0], instruction.operands[1]) as usize
        },
        Some('R') => {
            let ret:u16 = stack::pop_return_addr(state);
            println!("Getting return address: {:04x}", ret);
            ret as usize
        },
        _ => (state.pc)
    }
}

fn condition(state: &mut State, instruction: &Instruction, expected_condition: bool) -> AddressPtr {
    if expected_condition {
        default(state, instruction)
    }else{
        state.pc + instruction.size()
    }
}

pub fn carry(state: &mut State, instruction: &Instruction) -> AddressPtr {
    condition(state, instruction, state.cc.cy)
}

pub fn no_carry(state: &mut State, instruction: &Instruction) -> AddressPtr {
    condition(state, instruction, !state.cc.cy)
}

pub fn zero(state: &mut State, instruction: &Instruction) -> AddressPtr {
    condition(state, instruction, state.cc.z)
}

pub fn no_zero(state: &mut State, instruction: &Instruction) -> AddressPtr {
    condition(state, instruction, !state.cc.z)
}

pub fn minus(state: &mut State, instruction: &Instruction) -> AddressPtr {
    condition(state, instruction, state.cc.s)
}

pub fn positive(state: &mut State, instruction: &Instruction) -> AddressPtr {
    condition(state, instruction, !state.cc.s)
}

pub fn parity_even(state: &mut State, instruction: &Instruction) -> AddressPtr {
    condition(state, instruction, state.cc.p)
}

pub fn parity_odd(state: &mut State, instruction: &Instruction) -> AddressPtr {
    condition(state, instruction, !state.cc.p)
}

pub fn load_pc(state: &mut State, instruction: &Instruction) -> AddressPtr {
    println!("ERROR: {} has not been implemented!", instruction.to_string());
    state.pc
}