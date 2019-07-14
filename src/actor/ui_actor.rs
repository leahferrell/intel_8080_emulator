use std::sync::mpsc::{Receiver, Sender};

const EXIT_CODE: bool = true;

pub struct UiActor {
    pub exit_sender: Sender<bool>,
    pub input_sender: Sender<u8>,
    pub display_receiver: Receiver<u8>,
    pub audio_receiver: Receiver<u8>,
}

impl UiActor {
    pub fn new(
        exit_sender: Sender<bool>,
        input_sender: Sender<u8>,
        display_receiver: Receiver<u8>,
        audio_receiver: Receiver<u8>
    ) -> UiActor {
        UiActor {
            exit_sender,
            input_sender,
            display_receiver,
            audio_receiver,
        }
    }

    pub fn send_exit_code(&self){
        self.exit_sender.send(EXIT_CODE).unwrap();
    }
}