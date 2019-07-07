use crate::architecture::state::State;
use crate::architecture::instruction::Instruction;
use crate::architecture::registers::Register;


/// # Data Instructions:
///
/// Instructions which transfer data between registers or between
/// memory and registers.

pub fn move_reg(state: &mut State, instruction: &Instruction) -> bool {
    state.move_reg_to_reg(&instruction.register[0], &instruction.register[1]);

    state.pc += instruction.num_of_bytes();
    false
}

pub fn load_accumulator(state: &mut State, instruction: &Instruction) -> bool {
    let reg = &instruction.register[0];

    let addr = match reg {
        Register::B => (state.b, state.c),
        Register::D => (state.d, state.e),
        Register::H => (state.h, state.l),
        _ => (0,0),
    };

    let addr = (((addr.0 as u16) << 8) | addr.1 as u16) as usize;

    println!("Getting data from: #${:04x}",addr);

    let data = state.memory[addr];

    state.set_8bit_reg(&Register::A, data);

    state.pc += instruction.num_of_bytes();
    false
}

pub fn store_accumulator(state: &mut State, instruction: &Instruction) -> bool {
    println!("ERROR: {} has not been implemented!", instruction.to_string());
    true
}