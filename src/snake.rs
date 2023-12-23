use glam::{IVec2, UVec2};

use crate::direction::Direction;

pub struct Snake {
    parts: Vec<UVec2>,
    velocity: IVec2,
    should_grow: bool,
}

impl Snake {
    pub fn new(initial_head_position: UVec2) -> Self {
        Self {
            parts: vec![initial_head_position],
            velocity: IVec2::ZERO,
            should_grow: false,
        }
    }

    pub fn get_head(&self) -> UVec2 {
        self.parts[0]
    }

    pub fn get_parts(&self) -> &[UVec2] {
        &self.parts
    }

    pub fn turn(&mut self, direction: Direction) {
        let new_velocity = match direction {
            Direction::Up => IVec2::new(0, 1),
            Direction::Down => IVec2::new(0, -1),
            Direction::Left => IVec2::new(-1, 0),
            Direction::Right => IVec2::new(1, 0),
        };

        if (IVec2::ZERO != self.velocity)
            && (((0 == self.velocity.x) && (0 == new_velocity.x))
                || ((0 == self.velocity.y) && (0 == new_velocity.y)))
        {
            return;
        }

        self.velocity = new_velocity;
    }

    pub fn crawl(&mut self) {
        let new_head = (self.parts[0].as_ivec2() + self.velocity).as_uvec2();

        if self.should_grow {
            self.parts.push(UVec2::ZERO);
            self.should_grow = false;
        }

        for part_index in (1..self.parts.len()).rev() {
            self.parts[part_index] = self.parts[part_index - 1]
        }

        self.parts[0] = new_head;
    }

    pub fn detect_collision(&self) -> bool {
        if self.parts.len() <= 1 {
            return false;
        }

        self.parts[1..].iter().any(|&part| part == self.parts[0])
    }

    pub fn grow(&mut self) {
        self.should_grow = true
    }
}
