use crate::architecture::state::State;
use crate::architecture::instruction::Instruction;
use crate::architecture::registers::Register;


/// # Carry Bit Instructions:
///
/// Instructions which operate directly on the carry bit.

pub fn complement(state: &mut State, instruction: &Instruction) -> bool {
    state.cc.cy = !state.cc.cy;
    state.pc += instruction.num_of_bytes();
    false
}

pub fn set(state: &mut State, instruction: &Instruction) -> bool {
    state.cc.cy = true;
    state.pc += instruction.num_of_bytes();
    false
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::architecture::condition_codes::ConditionCodes;
    use crate::architecture::instruction::Instruction;
    use crate::architecture::opcodes::OpCode;

    #[test]
    fn test_complement() {
        let mut cc = ConditionCodes{z: false,s: false,p: false,cy: false,ac: false,pad: 0};
        let mut state = State {cc,..Default::default()};
        let instruction = &Instruction{opcode:OpCode::CMC,..Default::default()};

        let result = complement(&mut state, instruction);
        assert_eq!(result, false);
        assert_eq!(state.pc, 1);
        assert_eq!(state.cc.cy, true);
    }

    #[test]
    fn test_complement_while_already_set() {
        let mut cc = ConditionCodes{z: false,s: false,p: false,cy: true,ac: false,pad: 0};
        let mut state = State {cc,..Default::default()};
        let instruction = &Instruction{opcode:OpCode::CMC,..Default::default()};

        let result = complement(&mut state, instruction);
        assert_eq!(result, false);
        assert_eq!(state.pc, 1);
        assert_eq!(state.cc.cy, false);
    }

    #[test]
    fn test_set() {
        let mut cc = ConditionCodes{z: false,s: false,p: false,cy: false,ac: false,pad: 0};
        let mut state = State {cc,..Default::default()};
        let instruction = &Instruction{opcode:OpCode::STC,..Default::default()};

        let result = set(&mut state, instruction);
        assert_eq!(result, false);
        assert_eq!(state.pc, 1);
        assert_eq!(state.cc.cy, true);
    }

    #[test]
    fn test_set_while_already_set() {
        let mut cc = ConditionCodes{z: false,s: false,p: false,cy: true,ac: false,pad: 0};
        let mut state = State {cc,..Default::default()};
        let instruction = &Instruction{opcode:OpCode::STC,..Default::default()};

        let result = set(&mut state, instruction);
        assert_eq!(result, false);
        assert_eq!(state.pc, 1);
        assert_eq!(state.cc.cy, true);
    }
}