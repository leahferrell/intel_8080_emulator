use crate::condition_codes::ConditionCodes;

#[derive(Default)]
pub struct State {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub pc: usize,
    pub sp: usize,
    pub memory: Vec<u8>,
    pub cc: ConditionCodes,
    pub int_enable: u8
}