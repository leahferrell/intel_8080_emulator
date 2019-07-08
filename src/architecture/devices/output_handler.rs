use crate::architecture::model::state::State;
use crate::architecture::OutputSignal;

pub trait OutputHandler {
    fn handle(&self, state: &mut State, signal: OutputSignal);
}

pub struct DefaultHandler {}

impl OutputHandler for DefaultHandler {
    fn handle(&self, state: &mut State, signal: OutputSignal){
        println!("Default output handler #{:02x} just received the {:02x} signal", signal.device, signal.signal);
    }
}