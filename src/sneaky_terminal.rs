use crossbeam::channel::{self, Receiver, Sender};
use glam::UVec2;

use crate::{
    arena::Arena, direction::Direction, food::Food, input_event::InputEvent,
    input_receiver::InputReceiver, raw_terminal::RawTerminal, renderer::Renderer, snake::Snake,
    timer::Timer, vsync_awaiter::VsyncAwaiter,
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
    food: Food,
}

const ARENA_SIZE: UVec2 = UVec2::new(80, 30);
const GAME_UPDATE_HZ: u32 = 15;

impl SneakyTerminal {
    pub fn new() -> Self {
        let (input_event_sender, input_event_receiver): (Sender<InputEvent>, Receiver<InputEvent>) =
            channel::bounded(512);

        let arena = Arena::new(ARENA_SIZE);

        let snake = Snake::new(UVec2::new(ARENA_SIZE.x / 2, ARENA_SIZE.y / 2));

        let mut food = Food::new();
        food.respawn(&arena, &snake);

        Self {
            _raw_terminal: RawTerminal::new(),
            _input_receiver: InputReceiver::new(input_event_sender),
            input_event_receiver,
            frame_timer: Timer::new(),
            renderer: Renderer::new(arena.get_dimensions()),
            vsync_awaiter: VsyncAwaiter::new(GAME_UPDATE_HZ),
            arena,
            snake,
            food,
        }
    }

    pub fn run(&mut self) {
        let mut input_events = Vec::new();
        let mut running = true;

        while running {
            input_events.clear();
            self.process_input_events(&mut input_events);

            running = self.simulate(&input_events);

            self.render(running);

            self.vsync_awaiter.wait();

            self.frame_timer.tick();
        }
    }

    fn process_input_events(&self, input_events: &mut Vec<InputEvent>) {
        while !self.input_event_receiver.is_empty() {
            // TODO: Error handling.
            let input_event = self.input_event_receiver.recv().unwrap();
            input_events.push(input_event);
        }
    }

    fn simulate(&mut self, input_events: &[InputEvent]) -> bool {
        for input_event in input_events.iter() {
            match input_event {
                InputEvent::W => self.snake.turn(Direction::Up),
                InputEvent::S => self.snake.turn(Direction::Down),
                InputEvent::A => self.snake.turn(Direction::Left),
                InputEvent::D => self.snake.turn(Direction::Right),
                InputEvent::Q => return false,
            }
        }

        self.snake.crawl();

        if self.arena.detect_collision(&self.snake) {
            return false;
        }

        if self.snake.detect_collision() {
            return false;
        }

        if self.food.detect_collision(&self.snake) {
            self.snake.grow();
            self.food.respawn(&self.arena, &self.snake);
        }

        true
    }

    fn render(&mut self, running: bool) {
        self.renderer.begin_scene();

        self.renderer.draw_arena(&self.arena);

        if running {
            self.renderer.draw_food(&self.food);
            self.renderer.draw_snake(&self.snake);
        } else {
            self.renderer.draw_text_centered("Game Over!");
        }

        #[cfg(debug_assertions)]
        self.renderer.draw_text(
            &format!("{:.2} FPS", 1.0 / self.frame_timer.get_delta()),
            UVec2::new(2, self.arena.get_height() - 2),
        );

        self.renderer.end_scene();
    }
}

impl Default for SneakyTerminal {
    fn default() -> Self {
        Self::new()
    }
}
