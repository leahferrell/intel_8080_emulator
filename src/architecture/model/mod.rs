pub mod state;
pub mod registers;
pub mod condition_codes;
pub mod opcodes;
pub mod instruction;

type ConditionFlags = [bool];

type Registers = [u8];

enum Reg {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    M
}

enum Flag {
    CY,
    S1,
    P,
    S2,
    AC,
    S3,
    Z,
    S
}