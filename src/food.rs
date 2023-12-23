use glam::UVec2;
use rand::Rng;

use crate::{arena::Arena, snake::Snake};

pub struct Food {
    position: UVec2,
    icon: char,
}

impl Food {
    pub fn new() -> Self {
        Self {
            position: UVec2::ZERO,
            icon: ' ',
        }
    }

    pub fn get_position(&self) -> UVec2 {
        self.position
    }

    pub fn get_icon(&self) -> char {
        self.icon
    }

    pub fn detect_collision(&self, snake: &Snake) -> bool {
        self.position == snake.get_head()
    }

    pub fn respawn(&mut self, arena: &Arena, snake: &Snake) {
        self.position = Self::generate_position(arena, snake);
        self.icon = Self::generate_icon();
    }

    fn generate_position(arena: &Arena, snake: &Snake) -> UVec2 {
        let mut rng = rand::thread_rng();

        loop {
            let position = UVec2::new(
                rng.gen_range(1..(arena.get_width() - 1)),
                rng.gen_range(1..(arena.get_height() - 1)),
            );
            if !snake.get_parts().iter().any(|&part| part == position) {
                return position;
            }
        }
    }

    fn generate_icon() -> char {
        let mut rng = rand::thread_rng();
        rng.gen_range(b'A'..=b'Z') as char
    }
}
