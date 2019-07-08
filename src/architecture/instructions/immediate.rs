use crate::architecture::model::state::State;
use crate::architecture::model::instruction::Instruction;
use crate::architecture::units::memory_unit;
use crate::architecture::model::registers::Register;
use crate::architecture::units::arithmetic_logic_unit;

/// # Immediate Instructions:
///
/// Instructions which perform operations using a byte or bytes of data
/// which are part of the instruction itself.

pub fn load_register_pair(state: &mut State, instruction: &Instruction) -> usize {
    memory_unit::set_mem_loc_in_reg(state, &instruction.register[0], &instruction.operands);
    state.pc + instruction.size()
}

pub fn move_data(state: &mut State, instruction: &Instruction) -> usize {
    let reg = &instruction.register[0];
    let data = instruction.operands[0];

    memory_unit::set_reg_value(state, reg, data);
    state.pc + instruction.size()
}

pub fn add(state: &mut State, instruction: &Instruction) -> usize {
    println!("ERROR: {} has not been implemented!", instruction.to_string());
    state.pc
}

pub fn add_with_carry(state: &mut State, instruction: &Instruction) -> usize {
    println!("ERROR: {} has not been implemented!", instruction.to_string());
    state.pc
}

pub fn sub(state: &mut State, instruction: &Instruction) -> usize {
    println!("ERROR: {} has not been implemented!", instruction.to_string());
    state.pc
}

pub fn sub_with_borrow(state: &mut State, instruction: &Instruction) -> usize {
    println!("ERROR: {} has not been implemented!", instruction.to_string());
    state.pc
}

pub fn and(state: &mut State, instruction: &Instruction) -> usize {
    println!("ERROR: {} has not been implemented!", instruction.to_string());
    state.pc
}

pub fn xor(state: &mut State, instruction: &Instruction) -> usize {
    println!("ERROR: {} has not been implemented!", instruction.to_string());
    state.pc
}

pub fn or(state: &mut State, instruction: &Instruction) -> usize {
    println!("ERROR: {} has not been implemented!", instruction.to_string());
    state.pc
}

pub fn compare(state: &mut State, instruction: &Instruction) -> usize {
    let acc = memory_unit::get_reg_value(state, &Register::A);
    let imm = instruction.operands[0];

    let result = arithmetic_logic_unit::compare(acc, imm);

    state.cc.cy = result.1.cy;
    state.cc.s = result.1.s;
    state.cc.z = result.1.z;
    state.cc.p = result.1.p;
    state.cc.ac = result.1.ac;

    state.pc + instruction.size()
}