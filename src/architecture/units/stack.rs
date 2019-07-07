use crate::architecture::state::State;

pub fn push_return_addr(state: &mut State, ret: u16){
    state.memory[state.sp-1] = ((ret >> 8) & 0xff) as u8;
    state.memory[state.sp-2] = (ret & 0xff) as u8;
    state.sp -= 2;
}

pub fn pop_return_addr(state: &mut State) -> u16 {
    let addr1 = state.memory[state.sp+1];
    let addr2 = state.memory[state.sp];

    let ret = ((addr1 as u16) << 8) | addr2 as u16;

    state.sp += 2;

    ret
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