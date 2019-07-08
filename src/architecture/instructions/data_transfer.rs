use crate::architecture::model::state::State;
use crate::architecture::units::memory_unit;
use crate::architecture::model::instruction::Instruction;
use crate::architecture::model::registers::Register;
use crate::architecture::AddressPtr;

/// # Data Instructions:
///
/// Instructions which transfer data between registers or between
/// memory and registers.

pub fn move_reg(state: &mut State, instruction: &Instruction) -> AddressPtr {
    let value = memory_unit::get_reg_value(state, &instruction.register[1]);

    memory_unit::set_reg_value(state, &instruction.register[0], value);

    state.pc + instruction.size()
}

pub fn load_accumulator(state: &mut State, instruction: &Instruction) -> AddressPtr {
    let data = memory_unit::get_reg_mem_value(state, &instruction.register[0]);

    memory_unit::set_reg_value(state, &Register::A, data);

    state.pc + instruction.size()
}

pub fn store_accumulator(state: &mut State, instruction: &Instruction) -> AddressPtr {
    println!("ERROR: {} has not been implemented!", instruction.to_string());
    state.pc
}