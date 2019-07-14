use crate::architecture::models::state::State;
use crate::architecture::models::instruction::Instruction;
use crate::architecture::units::memory_unit;
use crate::architecture::models::registers::Register;
use crate::architecture::units::arithmetic_logic_unit;
use crate::architecture::AddressPtr;

/// # Immediate Instructions:
///
/// Instructions which perform operations using a byte or bytes of data
/// which are part of the instruction itself.

pub fn load_register_pair(state: &mut State, instruction: &Instruction) -> AddressPtr {
    memory_unit::set_mem_loc_in_reg(state, &instruction.register[0], &instruction.operands);
    state.pc + instruction.size()
}

pub fn move_data(state: &mut State, instruction: &Instruction) -> AddressPtr {
    let reg = &instruction.register[0];
    let data = instruction.operands[0];

    memory_unit::set_reg_value(state, reg, data);
    state.pc + instruction.size()
}

pub fn add(state: &mut State, instruction: &Instruction) -> AddressPtr {
    let acc_value = state.a;
    let imm_value = instruction.operands[0];

    let result = arithmetic_logic_unit::add(acc_value, imm_value);

    state.cc.cy = result.1.cy;
    state.cc.s = result.1.s;
    state.cc.z = result.1.z;
    state.cc.p = result.1.p;
    state.cc.ac = result.1.ac;

    state.a = result.0;

    state.pc + instruction.size()
}

//TODO: unit tests and verify logic on condition codes is correct
pub fn add_with_carry(state: &mut State, instruction: &Instruction) -> AddressPtr {
    let acc_value = state.a;
    let imm_value = instruction.operands[0];

    let result = arithmetic_logic_unit::add(acc_value, imm_value);

    let result = if state.cc.cy {arithmetic_logic_unit::add(result.0, 1)} else {result};

    state.cc.cy = result.1.cy;
    state.cc.s = result.1.s;
    state.cc.z = result.1.z;
    state.cc.p = result.1.p;
    state.cc.ac = result.1.ac;

    state.a = result.0;

    state.pc + instruction.size()
}

//TODO: unit tests
pub fn sub(state: &mut State, instruction: &Instruction) -> AddressPtr {
    let acc_value = state.a;
    let imm_value = instruction.operands[0];

    let result = arithmetic_logic_unit::sub(acc_value, imm_value);

    state.cc.cy = result.1.cy;
    state.cc.s = result.1.s;
    state.cc.z = result.1.z;
    state.cc.p = result.1.p;
    state.cc.ac = result.1.ac;

    state.a = result.0;

    state.pc + instruction.size()
}

//TODO: unit tests
pub fn sub_with_borrow(state: &mut State, instruction: &Instruction) -> AddressPtr {
    let acc_value = state.a;
    let imm_value = instruction.operands[0];

    let result = arithmetic_logic_unit::sub(acc_value, imm_value);

    let result = if state.cc.cy {arithmetic_logic_unit::sub(result.0, 1)} else {result};

    state.cc.cy = result.1.cy;
    state.cc.s = result.1.s;
    state.cc.z = result.1.z;
    state.cc.p = result.1.p;
    state.cc.ac = result.1.ac;

    state.a = result.0;

    state.pc + instruction.size()
}

//TODO: unit tests
pub fn and(state: &mut State, instruction: &Instruction) -> AddressPtr {
    let acc_value = state.a;
    let imm_value = instruction.operands[0];

    let result = arithmetic_logic_unit::and(acc_value, imm_value);

    state.cc.cy = result.1.cy;
    state.cc.s = result.1.s;
    state.cc.z = result.1.z;
    state.cc.p = result.1.p;

    state.a = result.0;

    state.pc + instruction.size()
}

//TODO: unit tests
pub fn xor(state: &mut State, instruction: &Instruction) -> AddressPtr {
    let acc_value = state.a;
    let imm_value = instruction.operands[0];

    let result = arithmetic_logic_unit::xor(acc_value, imm_value);

    state.cc.cy = result.1.cy;
    state.cc.s = result.1.s;
    state.cc.z = result.1.z;
    state.cc.p = result.1.p;

    state.a = result.0;

    state.pc + instruction.size()
}

//TODO: unit tests
pub fn or(state: &mut State, instruction: &Instruction) -> AddressPtr {
    let acc_value = state.a;
    let imm_value = instruction.operands[0];

    let result = arithmetic_logic_unit::or(acc_value, imm_value);

    state.cc.cy = result.1.cy;
    state.cc.s = result.1.s;
    state.cc.z = result.1.z;
    state.cc.p = result.1.p;

    state.a = result.0;

    state.pc + instruction.size()
}

//TODO: unit tests
pub fn compare(state: &mut State, instruction: &Instruction) -> AddressPtr {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::architecture::models::opcodes::OpCode;

    #[test]
    fn test_load_reg_pair() {
        let mut state = State {..Default::default()};
        let instruction = &Instruction{opcode:OpCode::LXI,register: vec![Register::B], operands:vec![0x00, 0x20],..Default::default()};

        let result = load_register_pair(&mut state, instruction);
        assert_eq!(result, 3);
        assert_eq!(state.b, 0x00);
        assert_eq!(state.c, 0x20)
    }

    #[test]
    fn test_load_sp() {
        let mut state = State {..Default::default()};
        let instruction = &Instruction{opcode:OpCode::LXI,register: vec![Register::SP], operands:vec![0x00, 0x20],..Default::default()};

        let result = load_register_pair(&mut state, instruction);
        assert_eq!(result, 3);
        assert_eq!(state.sp, 0x0020);
    }

    #[test]
    fn test_move_data_reg() {
        let mut state = State {..Default::default()};
        let instruction = &Instruction{opcode:OpCode::MVI,register: vec![Register::B], operands:vec![0x1c],..Default::default()};

        let result = move_data(&mut state, instruction);
        assert_eq!(result, 2);
        assert_eq!(state.b, 0x1c);
    }

    #[test]
    fn test_move_data_mem() {
        let index:usize = 0x0020;
        let mut memory: Vec<u8> = vec![0;100];
        let value:u8 = 0x1c;

        let mut state = State {h: 0x00, l: 0x20, memory,..Default::default()};
        let instruction = &Instruction{opcode:OpCode::MVI,register: vec![Register::M], operands:vec![value],..Default::default()};

        let result = move_data(&mut state, instruction);
        assert_eq!(result, 2);
        assert_eq!(state.memory[index], value);
    }

    #[test]
    fn test_add() {
        let mut state = State {a: 0x10,..Default::default()};
        let instruction = &Instruction{opcode:OpCode::ADI, operands:vec![0x1c],..Default::default()};

        let result = add(&mut state, instruction);
        assert_eq!(result, 2);
        assert_eq!(state.a, 0x2c);
    }

    #[test]
    fn test_add_carry_set() {
        let mut state = State {a: 0x10,..Default::default()};
        state.cc.cy = true;
        let instruction = &Instruction{opcode:OpCode::ACI, operands:vec![0x1c],..Default::default()};

        let result = add_with_carry(&mut state, instruction);
        assert_eq!(result, 2);
        assert_eq!(state.a, 0x2d);
    }

    #[test]
    fn test_add_carry_not_set() {
        let mut state = State {a: 0x10,..Default::default()};
        state.cc.cy = false;
        let instruction = &Instruction{opcode:OpCode::ACI, operands:vec![0x1c],..Default::default()};

        let result = add_with_carry(&mut state, instruction);
        assert_eq!(result, 2);
        assert_eq!(state.a, 0x2c);
    }
}