use crate::architecture::condition_codes::ConditionCodes;
use crate::architecture::state::State;
use crate::architecture::registers::Register;

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

pub fn get_mem_value(state: &State, loc: (u8, u8)) -> u8 {
    let addr = ((loc.0 as u16) << 8) | loc.1 as u16;
    state.memory[addr as usize]
}

pub fn set_mem_value(state: &mut State, loc: (u8, u8), value: u8) {
    let addr = ((loc.0 as u16) << 8) | loc.1 as u16;
    state.memory[addr as usize] = value;
}

pub fn get_reg_mem_value(state: &State, reg: &Register) -> u8 {
    let addr = match reg {
        Register::B => (state.b, state.c),
        Register::D => (state.d, state.e),
        Register::H => (state.h, state.l),
        _ => (0,0),
    };

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
