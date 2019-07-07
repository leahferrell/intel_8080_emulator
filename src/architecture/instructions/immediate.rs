use crate::architecture::state::State;
use crate::architecture::instruction::Instruction;
use crate::architecture::registers::Register;


/// # Immediate Instructions:
///
/// Instructions which perform operations using a byte or bytes of data
/// which are part of the instruction itself.

pub fn load_register_pair(state: &mut State, instruction: &Instruction) -> bool {
    let reg = &instruction.register[0];
    let data = &instruction.operands;

    match reg {
        Register::B => {
            state.b = data[0];
            state.c = data[1];
        },
        Register::D => {
            state.d = data[0];
            state.c = data[1];
        },
        Register::H => {
            state.h = data[0];
            state.l = data[1];
        },
        Register::SP => {
            state.sp = instruction.get_addr() as usize;
        },
        _ => ()
    };

    state.pc += instruction.num_of_bytes();
    false
}

pub fn move_data(state: &mut State, instruction: &Instruction) -> bool {
    let reg = &instruction.register[0];
    let data = instruction.operands[0];

    state.set_8bit_reg(reg, data);
    state.pc += instruction.num_of_bytes();
    false
}

pub fn add(state: &mut State, instruction: &Instruction) -> bool {
    println!("ERROR: {} has not been implemented!", instruction.to_string());
    true
}

pub fn add_with_carry(state: &mut State, instruction: &Instruction) -> bool {
    println!("ERROR: {} has not been implemented!", instruction.to_string());
    true
}

pub fn sub(state: &mut State, instruction: &Instruction) -> bool {
    println!("ERROR: {} has not been implemented!", instruction.to_string());
    true
}

pub fn sub_with_borrow(state: &mut State, instruction: &Instruction) -> bool {
    println!("ERROR: {} has not been implemented!", instruction.to_string());
    true
}

pub fn and(state: &mut State, instruction: &Instruction) -> bool {
    println!("ERROR: {} has not been implemented!", instruction.to_string());
    true
}

pub fn xor(state: &mut State, instruction: &Instruction) -> bool {
    println!("ERROR: {} has not been implemented!", instruction.to_string());
    true
}

pub fn or(state: &mut State, instruction: &Instruction) -> bool {
    println!("ERROR: {} has not been implemented!", instruction.to_string());
    true
}

pub fn compare(state: &mut State, instruction: &Instruction) -> bool {
    println!("ERROR: {} has not been implemented!", instruction.to_string());
    true
}