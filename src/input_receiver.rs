use std::{
    io,
    thread::{self, JoinHandle},
};

use byteorder::ReadBytesExt;
use crossbeam::channel::Sender;

use crate::input_event::InputEvent;

pub struct InputReceiver {
    _receiver_thread: Option<JoinHandle<()>>,
}

impl InputReceiver {
    pub fn new(event_sender: Sender<InputEvent>) -> Self {
        Self {
            _receiver_thread: Some(thread::spawn(move || Self::receive_input(event_sender))),
        }
    }

    fn receive_input(event_sender: Sender<InputEvent>) {
        let mut stdin = io::stdin();

        loop {
            // TODO: Error handling.
            let input = stdin.read_u8().unwrap();

            let Some(input_event) = Self::process_input(input) else {
                continue;
            };

            // TODO: Error handling.
            event_sender.send(input_event).unwrap();
        }
    }

    fn process_input(input: u8) -> Option<InputEvent> {
        match input {
            b'w' => Some(InputEvent::W),
            b's' => Some(InputEvent::S),
            b'a' => Some(InputEvent::A),
            b'd' => Some(InputEvent::D),
            b'q' => Some(InputEvent::Q),
            _ => None,
        }
    }
}
