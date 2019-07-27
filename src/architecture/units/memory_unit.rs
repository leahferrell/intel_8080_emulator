use crate::architecture::models::state::State;
use crate::architecture::models::registers::Register;
use crate::architecture::units::arithmetic_logic_unit;

pub fn get_reg_value(state: &State, reg: &Register) -> u8 {
    match reg {
        Register::A => state.a,
        Register::B => state.b,
        Register::C => state.c,
        Register::D => state.d,
        Register::E => state.e,
        Register::H => state.h,
        Register::L => state.l,
        Register::M => get_mem_value(state, (state.h, state.l)),
        _ => 0
    }
}

pub fn get_double_reg_value(state: &State, reg: &Register) -> (u8,u8) {
    match reg {
        Register::B => (state.b, state.c),
        Register::D => (state.d, state.e),
        Register::H => (state.h, state.l),
        Register::PSW => (state.a, state.cc.as_binary()),
        Register::SP => arithmetic_logic_unit::u16_to_u8(state.sp as u16),
        _ => (0,0)
    }
}

pub fn set_double_reg_value(state: &mut State, reg: &Register, value: (u8,u8)) {
    match reg {
        Register::B => {state.b = value.0; state.c = value.1; },
        Register::D => {state.d = value.0; state.e = value.1; },
        Register::H => {state.h = value.0; state.l = value.1; },
        Register::PSW => {state.a = value.0; state.cc.update_from_binary(value.1); },
        Register::SP => {
            state.sp = arithmetic_logic_unit::u8_to_u16(value.0, value.1) as usize;
        },
        _ => ()
    };
}

pub fn get_mem_value(state: &State, loc: (u8, u8)) -> u8 {
    let addr = ((loc.0 as u16) << 8) | loc.1 as u16;
    state.memory[addr as usize]
}

pub fn set_mem_value(state: &mut State, loc: (u8, u8), value: u8) {
    let addr = ((loc.0 as u16) << 8) | loc.1 as u16;
    state.memory[addr as usize] = value;
}

pub fn get_reg_mem_value(state: &State, reg: &Register) -> u8 {
    let addr = get_double_reg_value(state, reg);
    get_mem_value(state, addr)
}

pub fn set_reg_mem_value(state: &mut State, reg: &Register, value: u8) {
    let addr = match reg {
        Register::B => (state.b, state.c),
        Register::D => (state.d, state.e),
        Register::H => (state.h, state.l),
        _ => (0,0),
    };

    set_mem_value(state, addr, value)
}

pub fn set_reg_value(state: &mut State, reg: &Register, value: u8) {
    match reg {
        Register::A => state.a = value,
        Register::B => state.b = value,
        Register::C => state.c = value,
        Register::D => state.d = value,
        Register::E => state.e = value,
        Register::H => state.h = value,
        Register::L => state.l = value,
        Register::M => set_mem_value(state, (state.h, state.l), value),
        _ => ()
    }
}

pub fn set_mem_loc_in_reg(state: &mut State, reg: &Register, data: &Vec<u8>) {
    match reg {
        Register::B => {
            state.b = data[0];
            state.c = data[1];
        },
        Register::D => {
            state.d = data[0];
            state.c = data[1];
        },
        Register::H => {
            state.h = data[0];
            state.l = data[1];
        },
        Register::SP => {
            state.sp = (((data[0] as u16) << 8) | data[1] as u16) as usize;
        },
        _ => ()
    };
}
