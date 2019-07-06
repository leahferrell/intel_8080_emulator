use architecture::isa;
use architecture::instruction::Instruction;
use architecture::condition_codes::ConditionCodes;
use architecture::state::State;

pub fn unimplemented_instruction(state: &mut State){
    println!("Error: Unimplemented instruction");
}

pub fn emulate(state: &mut State){
    let opcode = state.memory[state.pc];
    let mem_end = state.memory.len();
    let mut program_exit: bool = false;

    while state.pc < mem_end && !program_exit {
        let instr = isa::read_next(&state.memory, state.pc);
        state.pc += instr.num_of_bytes();
        program_exit = execute_instruction(state, instr);
    }
}

pub fn execute_instruction(state: &mut State, instruction: Instruction) -> bool {

    unimplemented_instruction(state);
    true
}