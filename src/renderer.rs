use std::io::{self, Stdout, Write};

use glam::UVec2;

use crate::arena::Arena;

pub struct Renderer {
    screen_dimensions: UVec2,
    render_buffer: Vec<char>,
    first_frame: bool,
    stdout: Stdout,
}

impl Renderer {
    pub fn new(screen_dimensions: UVec2) -> Self {
        Self {
            screen_dimensions,
            render_buffer: vec![' '; (screen_dimensions.y * screen_dimensions.x) as usize],
            first_frame: true,
            stdout: io::stdout(),
        }
    }

    pub fn begin_scene(&mut self) {
        if self.first_frame {
            self.first_frame = false;
            return;
        }

        self.reset_cursor();
    }

    pub fn end_scene(&mut self) {
        self.flush();
        self.stdout.flush().unwrap();
    }

    pub fn draw_arena(&mut self, arena: &Arena) {
        self.draw_horizontal_border(0, arena.get_width());
        self.draw_vertical_border(0, arena.get_height());
        self.draw_vertical_border(arena.get_width() - 1, arena.get_height());
        self.draw_horizontal_border(arena.get_height() - 1, arena.get_width());
    }

    fn reset_cursor(&mut self) {
        write!(self.stdout, "\x1B[{}F", self.screen_dimensions.y).unwrap();
        self.stdout.write_all(b"\r").unwrap();
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
