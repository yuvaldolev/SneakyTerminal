use std::{
    io::{self, Read},
    thread::{self, JoinHandle},
};

use crossbeam::channel::Sender;
use nix::sys::termios::{self, LocalFlags, SetArg};

use crate::input_event::InputEvent;

pub struct InputReceiver {
    receiver_thread: Option<JoinHandle<()>>,
}

impl InputReceiver {
    pub fn new(event_sender: Sender<InputEvent>) -> Self {
        // Ensure non-canonical input.
        Self::set_non_canonical_input();

        Self {
            receiver_thread: Some(thread::spawn(move || Self::receive_input(event_sender))),
        }
    }

    fn set_non_canonical_input() {
        // TODO: Error handling
        let mut stdin_termios = termios::tcgetattr(io::stdin()).unwrap();
        stdin_termios.local_flags.remove(LocalFlags::ICANON);
        termios::tcsetattr(io::stdin(), SetArg::TCSANOW, &stdin_termios).unwrap();
    }

    fn receive_input(event_sender: Sender<InputEvent>) {
        loop {
            let Some(input) = io::stdin().bytes().next().and_then(|result| result.ok()) else {
                continue;
            };

            let Some(input_event) = Self::process_input(input) else {
                continue;
            };

            // TODO: Error handling.
            event_sender.send(input_event).unwrap();
        }
    }

    fn process_input(input: u8) -> Option<InputEvent> {
        match input {
            b'w' => Some(InputEvent::Up),
            b's' => Some(InputEvent::Down),
            b'a' => Some(InputEvent::Left),
            b'd' => Some(InputEvent::Right),
            _ => None,
        }
    }
}
