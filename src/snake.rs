use glam::Vec2;

use crate::direction::Direction;

pub struct Snake {
    parts: Vec<Vec2>,
    velocity: Vec2,
}

const DIRECTIONAL_VELOCITY: f32 = 15.0;

impl Snake {
    pub fn new(initial_head_position: Vec2) -> Self {
        Self {
            parts: vec![
                initial_head_position,
                initial_head_position - Vec2::new(2.0, 0.0),
                initial_head_position - Vec2::new(4.0, 0.0),
                initial_head_position - Vec2::new(6.0, 0.0),
                initial_head_position - Vec2::new(8.0, 0.0),
            ],
            velocity: Vec2::ZERO,
        }
    }

    pub fn turn(&mut self, direction: Direction) {
        match direction {
            Direction::Up => {
                self.velocity.x = 0.0;
                self.velocity.y = DIRECTIONAL_VELOCITY;
            }
            Direction::Down => {
                self.velocity.x = 0.0;
                self.velocity.y = -DIRECTIONAL_VELOCITY;
            }
            Direction::Left => {
                self.velocity.x = -DIRECTIONAL_VELOCITY;
                self.velocity.y = 0.0;
            }
            Direction::Right => {
                self.velocity.x = DIRECTIONAL_VELOCITY;
                self.velocity.y = 0.0;
            }
        }
    }

    pub fn crawl(&mut self, delta_time: f32) {
        // Determine the new head position.
        let mut head = *self.parts.first().unwrap();
        let previous_head = head;
        head += self.velocity * delta_time;

        // Update all non-head part positions.
        if (head.as_uvec2() != previous_head.as_uvec2()) && (self.parts.len() > 1) {
            for part_index in (0..(self.parts.len() - 1)).rev() {
                self.parts[part_index + 1] = self.parts[part_index];
            }
        }

        // Update the head position.
        *self.parts.first_mut().unwrap() = head;
    }

    pub fn get_length(&self) -> usize {
        self.parts.len()
    }

    pub fn get_part(&self, index: usize) -> Vec2 {
        self.parts[index]
    }
}
