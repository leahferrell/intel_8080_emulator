use crate::architecture::models::state::State;
use crate::architecture::IoSignal;

pub trait OutputHandler {
    fn handle(&self, state: &mut State, signal: IoSignal);
}

pub struct DefaultHandler {}

impl OutputHandler for DefaultHandler {
    fn handle(&self, state: &mut State, signal: IoSignal){
        println!("Default output handler #{:02x} just received the {:02x} signal", signal.device, signal.signal);
    }
}