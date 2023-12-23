use glam::UVec2;

use crate::snake::Snake;

pub struct Arena {
    dimensions: UVec2,
}

impl Arena {
    pub fn new(size: UVec2) -> Self {
        Self { dimensions: size }
    }

    pub fn get_dimensions(&self) -> UVec2 {
        self.dimensions
    }

    pub fn get_width(&self) -> u32 {
        self.dimensions.x
    }

    pub fn get_height(&self) -> u32 {
        self.dimensions.y
    }

    pub fn detect_collision(&self, snake: &Snake) -> bool {
        let snake_head = snake.get_head();

        (0 == snake_head.x)
            || ((self.dimensions.x - 1) == snake_head.x)
            || (0 == snake_head.y)
            || ((self.dimensions.y - 1) == snake_head.y)
    }
}
