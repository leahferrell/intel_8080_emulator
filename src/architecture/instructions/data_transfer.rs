use crate::architecture::model::state::State;
use crate::architecture::units::memory_unit;
use crate::architecture::model::instruction::Instruction;
use crate::architecture::model::registers::Register;

/// # Data Instructions:
///
/// Instructions which transfer data between registers or between
/// memory and registers.

pub fn move_reg(state: &mut State, instruction: &Instruction) -> bool {
    let value = memory_unit::get_reg_value(state, &instruction.register[1]);

    memory_unit::set_reg_value(state, &instruction.register[0], value);

    state.pc += instruction.num_of_bytes();
    false
}

pub fn load_accumulator(state: &mut State, instruction: &Instruction) -> bool {
    let data = memory_unit::get_reg_mem_value(state, &instruction.register[0]);

    memory_unit::set_reg_value(state, &Register::A, data);

    state.pc += instruction.num_of_bytes();
    false
}

pub fn store_accumulator(state: &mut State, instruction: &Instruction) -> bool {
    println!("ERROR: {} has not been implemented!", instruction.to_string());
    true
}