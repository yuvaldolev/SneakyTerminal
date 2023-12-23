use std::thread::{self, JoinHandle};

use crossbeam::channel::Sender;

use crate::input_event::InputEvent;

pub struct InputReceiver {
    receiver_thread: Option<JoinHandle<()>>,
}

impl InputReceiver {
    pub fn new(event_sender: Sender<InputEvent>) -> Self {
        Self {
            receiver_thread: Some(thread::spawn(move || Self::receive_input(event_sender))),
        }
    }

    fn receive_input(event_sender: Sender<InputEvent>) {}
}
