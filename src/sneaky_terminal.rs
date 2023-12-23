use crossbeam::channel::{self, Receiver, Sender};

use crate::{input_event::InputEvent, input_receiver::InputReceiver};

pub struct SneakyTerminal {
    input_receiver: InputReceiver,
    input_event_receiver: Receiver<InputEvent>,
}

impl SneakyTerminal {
    pub fn new() -> Self {
        let (input_event_sender, input_event_receiver): (Sender<InputEvent>, Receiver<InputEvent>) =
            channel::bounded(512);

        Self {
            input_receiver: InputReceiver::new(input_event_sender),
            input_event_receiver,
        }
    }

    pub fn run(&self) {
        loop {
            // Process input events.
            while !self.input_event_receiver.is_empty() {
                let input_event = self.input_event_receiver.recv().unwrap();
                println!("Input Event: {input_event:?}");
            }

            // Simulate the game.

            // Render the game.
        }
    }
}
