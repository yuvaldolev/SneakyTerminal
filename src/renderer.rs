use std::io::{self, Stdout, Write};

use glam::UVec2;

use crate::{
    arena::Arena,
    colored_character::{self, ColoredCharacter},
    food::Food,
    snake::Snake,
};

pub struct Renderer {
    screen_dimensions: UVec2,
    render_buffer: Vec<ColoredCharacter>,
    stdout: Stdout,
}

impl Renderer {
    pub fn new(screen_dimensions: UVec2) -> Self {
        Self {
            screen_dimensions,
            render_buffer: vec![
                ColoredCharacter::new_uncolored(' ');
                (screen_dimensions.y * screen_dimensions.x) as usize
            ],
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
        self.draw_colored_character(
            ColoredCharacter::new_colored('█', colored_character::RED_COLOR),
            snake.get_head(),
        );

        if snake.get_parts().len() <= 1 {
            return;
        }

        for part in snake.get_parts()[1..].iter() {
            self.draw_character('█', *part);
        }
    }

    pub fn draw_food(&mut self, food: &Food) {
        self.draw_character(food.get_icon(), food.get_position());
    }

    pub fn draw_text(&mut self, text: &str, position: UVec2) {
        for (index, character) in text.chars().enumerate() {
            self.draw_character(character, position + UVec2::new(index as u32, 0));
        }
    }

    pub fn draw_text_centered(&mut self, text: &str) {
        let text_width = text.len();
        let text_draw_x = (self.screen_dimensions.x / 2) - ((text_width as u32) / 2);

        self.draw_text(text, UVec2::new(text_draw_x, self.screen_dimensions.y / 2));
    }

    fn reset_cursor(&mut self) {
        self.stdout.write_all(b"\x1B[H").unwrap();
    }

    fn flush(&mut self) {
        for y in (0..self.screen_dimensions.y).rev() {
            for x in 0..self.screen_dimensions.x {
                let character =
                    &self.render_buffer[self.render_buffer_index_from_position(UVec2::new(x, y))];
                self.stdout.write_all(character.get_color()).unwrap();
                write!(self.stdout, "{}", character.get_character()).unwrap();
            }

            self.stdout.write_all(b"\x1B[1E").unwrap();
            self.stdout
                .write_all(colored_character::DEFAULT_COLOR)
                .unwrap();
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
        self.draw_colored_character(ColoredCharacter::new_uncolored(character), position);
    }

    fn draw_colored_character(&mut self, character: ColoredCharacter, position: UVec2) {
        let render_buffer_index = self.render_buffer_index_from_position(position);
        self.render_buffer[render_buffer_index] = character;
    }

    fn render_buffer_index_from_position(&self, position: UVec2) -> usize {
        ((position.y * self.screen_dimensions.x) + position.x) as usize
    }
}
