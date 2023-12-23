use std::io::{self, Stdout, Write};

use glam::UVec2;

use crate::{arena::Arena, snake::Snake};

pub struct Renderer {
    screen_dimensions: UVec2,
    render_buffer: Vec<char>,
    stdout: Stdout,
}

impl Renderer {
    pub fn new(screen_dimensions: UVec2) -> Self {
        Self {
            screen_dimensions,
            render_buffer: vec![' '; (screen_dimensions.y * screen_dimensions.x) as usize],
            stdout: io::stdout(),
        }
    }

    pub fn begin_scene(&mut self) {
        self.reset_cursor();
    }

    pub fn end_scene(&mut self) {
        self.flush();
        self.stdout.flush().unwrap();
    }

    pub fn draw_arena(&mut self, arena: &Arena) {
        // Draw the borders.
        self.draw_horizontal_border(0, arena.get_width());
        self.draw_vertical_border(0, arena.get_height());
        self.draw_vertical_border(arena.get_width() - 1, arena.get_height());
        self.draw_horizontal_border(arena.get_height() - 1, arena.get_width());

        // Draw the reset of the arena as spaces.
        for y in 1..(arena.get_height() - 1) {
            for x in 1..(arena.get_width() - 1) {
                self.draw_character(' ', UVec2::new(x, y));
            }
        }
    }

    pub fn draw_snake(&mut self, snake: &Snake) {
        for part_index in 0..snake.get_length() {
            let part = snake.get_part(part_index);

            let left_half = part.as_uvec2();
            let right_half = left_half + UVec2::new(1, 0);

            self.draw_character('█', left_half);
            self.draw_character('█', right_half);
        }
    }

    pub fn draw_text(&mut self, text: &str, position: UVec2) {
        for (index, character) in text.chars().enumerate() {
            self.draw_character(character, position + UVec2::new(index as u32, 0));
        }
    }

    fn reset_cursor(&mut self) {
        self.stdout.write_all(b"\x1B[H").unwrap();
    }

    fn flush(&mut self) {
        for y in (0..self.screen_dimensions.y).rev() {
            for x in 0..self.screen_dimensions.x {
                write!(
                    self.stdout,
                    "{}",
                    self.render_buffer[self.render_buffer_index_from_position(UVec2::new(x, y))]
                )
                .unwrap();
            }

            self.stdout.write_all(b"\x1B[1E").unwrap();
        }
    }

    fn draw_horizontal_border(&mut self, y: u32, width: u32) {
        for x in 0..width {
            self.draw_character('-', UVec2::new(x, y));
        }
    }

    fn draw_vertical_border(&mut self, x: u32, height: u32) {
        for y in 1..height {
            self.draw_character('|', UVec2::new(x, y));
        }
    }

    fn draw_character(&mut self, character: char, position: UVec2) {
        let render_buffer_index = self.render_buffer_index_from_position(position);
        self.render_buffer[render_buffer_index] = character;
    }

    fn render_buffer_index_from_position(&self, position: UVec2) -> usize {
        ((position.y * self.screen_dimensions.x) + position.x) as usize
    }
}
