use crate::architecture::model::state::State;
use crate::architecture::model::registers::Register;

pub fn push_return_addr(state: &mut State, ret: u16){
    let ret1 = ((ret >> 8) & 0xff) as u8;
    let ret2 = (ret & 0xff) as u8;
    push(state, (ret1, ret2));
}

pub fn pop_return_addr(state: &mut State) -> u16 {
    let addr = pop(state);
    ((addr.0 as u16) << 8) | addr.1 as u16
}

pub fn push_reg(state: &mut State, reg: &Register){
    let addr = match reg {
        Register::PSW => (state.a, state.cc.as_binary()),
        Register::B => (state.b, state.c),
        Register::D => (state.d, state.e),
        Register::H => (state.h, state.l),
        _ => (0,0)
    };

    push(state, addr);
}

pub fn push(state: &mut State, addr: (u8, u8)){
    state.memory[state.sp-1] = addr.0;
    state.memory[state.sp-2] = addr.1;
    state.sp -= 2;
}

pub fn pop(state: &mut State) -> (u8, u8) {
    let addr1 = state.memory[state.sp+1];
    let addr2 = state.memory[state.sp];

    state.sp += 2;
    (addr1, addr2)
}

pub fn pop_to_reg(state: &mut State, reg: &Register){
    let addr = pop(state);

    match reg {
        Register::PSW => {state.a = addr.0; state.cc.update_from_binary(addr.1)},
        Register::B => {state.b = addr.0; state.c = addr.1},
        Register::D => {state.d = addr.0; state.e = addr.1},
        Register::H => {state.h = addr.0; state.l = addr.1},
        _ => ()
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push() {
        let mut state = &mut State{pc:0x2000,sp:0x1002,memory:vec![0;10000],..Default::default()};
        let ret:u16 = (state.pc + 3) as u16;
        push_return_addr(state, ret);
        assert_eq!(state.pc, 0x2000);
        assert_eq!(state.sp, 0x1000);
        assert_eq!(state.memory[state.sp+1],0x20);
        assert_eq!(state.memory[state.sp],0x03);
    }

    #[test]
    fn test_pop() {
        let mut state = &mut State{pc:0x2000,sp:0x1002,memory:vec![0;10000],..Default::default()};
        let ret:u16 = (state.pc + 3) as u16;
        push_return_addr(state, ret);
        let popped_ret = pop_return_addr(state);
        assert_eq!(state.pc, 0x2000);
        assert_eq!(state.sp, 0x1002);
        assert_eq!(popped_ret,0x2003);
    }
}