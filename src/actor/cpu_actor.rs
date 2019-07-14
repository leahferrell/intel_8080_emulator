use std::sync::mpsc::{Receiver, Sender};

pub struct CpuActor {
    pub exit_receiver: Receiver<bool>,
    pub input_receiver: Receiver<u8>,
    pub display_sender: Sender<u8>,
    pub audio_sender: Sender<u8>,
}

impl CpuActor {
    pub fn new(
        exit_receiver: Receiver<bool>,
        input_receiver: Receiver<u8>,
        display_sender: Sender<u8>,
        audio_sender: Sender<u8>
    ) -> CpuActor {
        CpuActor {
            exit_receiver,
            input_receiver,
            display_sender,
            audio_sender,
        }
    }

    pub fn should_exit(&self) -> bool {
        self.exit_receiver.recv().unwrap_or(false)
    }
}