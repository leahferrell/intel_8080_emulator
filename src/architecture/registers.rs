extern crate strum;
extern crate strum_macros;

use strum_macros::Display;

#[derive(Display, Debug)]
pub enum Register {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    M,
    SP,
    PSW,
    #[strum(serialize="0")]
    _0,
    #[strum(serialize="1")]
    _1,
    #[strum(serialize="2")]
    _2,
    #[strum(serialize="3")]
    _3,
    #[strum(serialize="4")]
    _4,
    #[strum(serialize="5")]
    _5,
    #[strum(serialize="6")]
    _6,
    #[strum(serialize="7")]
    _7
}