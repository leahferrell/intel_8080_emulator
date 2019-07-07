use crate::architecture::state::State;
use crate::architecture::instruction::Instruction;
use crate::architecture::registers::Register;
use crate::architecture::units::memory_unit;


/// # Immediate Instructions:
///
/// Instructions which perform operations using a byte or bytes of data
/// which are part of the instruction itself.

pub fn load_register_pair(state: &mut State, instruction: &Instruction) -> bool {
    let reg = &instruction.register[0];
    let data = &instruction.operands;

    memory_unit::set_mem_loc_in_reg(state, &instruction.register[0], &instruction.operands);

    state.pc += instruction.num_of_bytes();
    false
}

pub fn move_data(state: &mut State, instruction: &Instruction) -> bool {
    let reg = &instruction.register[0];
    let data = instruction.operands[0];

    memory_unit::set_reg_value(state, reg, data);
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