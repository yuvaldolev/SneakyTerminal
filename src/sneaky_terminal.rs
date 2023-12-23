use crossbeam::channel::{self, Receiver, Sender};
use glam::UVec2;

use crate::{
    arena::Arena, input_event::InputEvent, input_receiver::InputReceiver,
    raw_terminal::RawTerminal, renderer::Renderer, timer::Timer,
};

pub struct SneakyTerminal {
    _raw_terminal: RawTerminal,
    _input_receiver: InputReceiver,
    input_event_receiver: Receiver<InputEvent>,
    timer: Timer,
    renderer: Renderer,
    arena: Arena,
}

const ARENA_SIZE: UVec2 = UVec2::new(150, 50);

impl SneakyTerminal {
    pub fn new() -> Self {
        let (input_event_sender, input_event_receiver): (Sender<InputEvent>, Receiver<InputEvent>) =
            channel::bounded(512);

        Self {
            _raw_terminal: RawTerminal::new(),
            _input_receiver: InputReceiver::new(input_event_sender),
            input_event_receiver,
            timer: Timer::new(),
            renderer: Renderer::new(ARENA_SIZE),
            arena: Arena::new(ARENA_SIZE),
        }
    }

    pub fn run(&mut self) {
        let mut input_events = Vec::new();

        loop {
            // Process input events.
            input_events.clear();
            if !self.process_input_events(&mut input_events) {
                break;
            }

            // Simulate the game.
            self.simulate(&input_events);

            // Render the game.
            self.render();
        }
    }

    fn process_input_events(&self, input_events: &mut Vec<InputEvent>) -> bool {
        while !self.input_event_receiver.is_empty() {
            // TODO: Error handling.
            let input_event = self.input_event_receiver.recv().unwrap();

            if InputEvent::Quit == input_event {
                return false;
            }

            // TODO: Error handling.
            input_events.push(input_event);
        }

        true
    }

    fn simulate(&mut self, input_events: &[InputEvent]) {
        let delta_time = self.timer.measure_delta();
        // snake.simulate(input_events, delta_time);
    }

    fn render(&mut self) {
        self.renderer.begin_scene();
        self.renderer.draw_arena(&self.arena);
        self.renderer.end_scene();
    }
}

impl Default for SneakyTerminal {
    fn default() -> Self {
        Self::new()
    }
}
