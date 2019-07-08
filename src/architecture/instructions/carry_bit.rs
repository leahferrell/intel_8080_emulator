use crate::architecture::model::state::State;
use crate::architecture::model::instruction::Instruction;

/// # Carry Bit Instructions:
///
/// Instructions which operate directly on the carry bit.

pub fn complement(state: &mut State, instruction: &Instruction)-> usize {
    state.cc.cy = !state.cc.cy;
    state.pc + instruction.size()
}

pub fn set(state: &mut State, instruction: &Instruction)-> usize {
    state.cc.cy = true;
    state.pc + instruction.size()
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::architecture::model::condition_codes::ConditionCodes;
    use crate::architecture::model::opcodes::OpCode;

    #[test]
    fn test_complement() {
        let cc = ConditionCodes{z: false,s: false,p: false,cy: false,ac: false};
        let mut state = State {cc,..Default::default()};
        let instruction = &Instruction{opcode:OpCode::CMC,..Default::default()};

        let result = complement(&mut state, instruction);
        assert_eq!(result, 1);
        assert_eq!(state.cc.cy, true);
    }

    #[test]
    fn test_complement_while_already_set() {
        let cc = ConditionCodes{z: false,s: false,p: false,cy: true,ac: false};
        let mut state = State {cc,..Default::default()};
        let instruction = &Instruction{opcode:OpCode::CMC,..Default::default()};

        let result = complement(&mut state, instruction);
        assert_eq!(result, 1);
        assert_eq!(state.cc.cy, false);
    }

    #[test]
    fn test_set() {
        let cc = ConditionCodes{z: false,s: false,p: false,cy: false,ac: false};
        let mut state = State {cc,..Default::default()};
        let instruction = &Instruction{opcode:OpCode::STC,..Default::default()};

        let result = set(&mut state, instruction);
        assert_eq!(result, 1);
        assert_eq!(state.cc.cy, true);
    }

    #[test]
    fn test_set_while_already_set() {
        let cc = ConditionCodes{z: false,s: false,p: false,cy: true,ac: false};
        let mut state = State {cc,..Default::default()};
        let instruction = &Instruction{opcode:OpCode::STC,..Default::default()};

        let result = set(&mut state, instruction);
        assert_eq!(result, 1);
        assert_eq!(state.cc.cy, true);
    }
}