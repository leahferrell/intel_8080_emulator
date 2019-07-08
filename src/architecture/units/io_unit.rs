use crate::architecture::devices::output_handler;
use crate::architecture::devices::output_handler::OutputHandler;

pub fn get_handler(device_id: u8) -> &'static OutputHandler {
    &output_handler::DefaultHandler{}
}