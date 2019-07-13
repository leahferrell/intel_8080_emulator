use crate::architecture::model::state::State;
use crate::architecture::IoSignal;

pub trait InputHandler {
    fn next_signal(&self, state: &mut State) -> IoSignal;
}

pub struct DefaultHandler {}

impl InputHandler for DefaultHandler {
    fn next_signal(&self, state: &mut State) -> IoSignal {
        let signal = IoSignal{device: 0x10, signal: 0x1c};
        println!("Default output handler #{:02x} just received the {:02x} signal", signal.device, signal.signal);
        signal
    }
}