use crate::architecture::devices::output_handler;
use crate::architecture::devices::output_handler::OutputHandler;
use crate::architecture::devices::input_handler::InputHandler;
use crate::architecture::devices::input_handler;

pub fn get_output_handler(device_id: u8) -> &'static OutputHandler {
    &output_handler::DefaultHandler{}
}

pub fn get_input_handler(device_id: u8) -> &'static InputHandler {
    &input_handler::DefaultHandler{}
}