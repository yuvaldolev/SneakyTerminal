use crossbeam::channel::{self, Receiver, Sender};
use glam::{UVec2, Vec2};

use crate::{
    arena::Arena, direction::Direction, input_event::InputEvent, input_receiver::InputReceiver,
    raw_terminal::RawTerminal, renderer::Renderer, snake::Snake, timer::Timer,
    vsync_awaiter::VsyncAwaiter,
};

pub struct SneakyTerminal {
    _raw_terminal: RawTerminal,
    _input_receiver: InputReceiver,
    input_event_receiver: Receiver<InputEvent>,
    frame_timer: Timer,
    renderer: Renderer,
    vsync_awaiter: VsyncAwaiter,
    arena: Arena,
    snake: Snake,
}

const ARENA_SIZE: UVec2 = UVec2::new(150, 50);
const GAME_UPDATE_HZ: u32 = 60;

impl SneakyTerminal {
    pub fn new() -> Self {
        let (input_event_sender, input_event_receiver): (Sender<InputEvent>, Receiver<InputEvent>) =
            channel::bounded(512);

        Self {
            _raw_terminal: RawTerminal::new(),
            _input_receiver: InputReceiver::new(input_event_sender),
            input_event_receiver,
            frame_timer: Timer::new(),
            renderer: Renderer::new(ARENA_SIZE),
            vsync_awaiter: VsyncAwaiter::new(GAME_UPDATE_HZ),
            arena: Arena::new(ARENA_SIZE),
            snake: Snake::new(Vec2::new(
                (ARENA_SIZE.x as f32) / 2.0,
                (ARENA_SIZE.y as f32) / 2.0,
            )),
        }
    }

    pub fn run(&mut self) {
        let mut input_events = Vec::new();

        loop {
            // Begin work.
            input_events.clear();
            if !self.process_input_events(&mut input_events) {
                break;
            }

            self.simulate(&input_events);

            self.render();

            self.vsync_awaiter.wait();

            self.frame_timer.tick();
        }
    }

    fn process_input_events(&self, input_events: &mut Vec<InputEvent>) -> bool {
        while !self.input_event_receiver.is_empty() {
            // TODO: Error handling.
            let input_event = self.input_event_receiver.recv().unwrap();

            if InputEvent::Q == input_event {
                return false;
            }

            // TODO: Error handling.
            input_events.push(input_event);
        }

        true
    }

    fn simulate(&mut self, input_events: &[InputEvent]) {
        for input_event in input_events.iter() {
            match input_event {
                InputEvent::W => self.snake.turn(Direction::Up),
                InputEvent::S => self.snake.turn(Direction::Down),
                InputEvent::A => self.snake.turn(Direction::Left),
                InputEvent::D => self.snake.turn(Direction::Right),
                _ => {}
            }
        }

        self.snake.crawl(self.frame_timer.get_delta());
    }

    fn render(&mut self) {
        self.renderer.begin_scene();
        self.renderer.draw_arena(&self.arena);
        self.renderer.draw_text(
            &format!("{} FPS", 1.0 / self.frame_timer.get_delta()),
            UVec2::new(2, self.arena.get_height() - 2),
        );
        self.renderer.draw_snake(&self.snake);
        self.renderer.end_scene();
    }
}

impl Default for SneakyTerminal {
    fn default() -> Self {
        Self::new()
    }
}
