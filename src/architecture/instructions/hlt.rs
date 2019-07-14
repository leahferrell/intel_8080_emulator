use crate::architecture::models::state::State;
use crate::architecture::models::instruction::Instruction;
use crate::architecture::AddressPtr;

/// # Halt Instructions:
///
/// Instructions which operate directly upon the Interrupt Enable
/// flip-flop INTE.

pub fn halt(state: &mut State, instruction: &Instruction) -> AddressPtr {
    state.stopped = true;
    state.pc + instruction.size()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::architecture::models::opcodes::OpCode;

    #[test]
    fn test_halt() {
        let mut state = State {..Default::default()};
        let instruction = &Instruction{opcode:OpCode::HLT,..Default::default()};
        let result = halt(&mut state, instruction);
        assert_eq!(result, 1);
        assert_eq!(state.stopped, true);
    }
}