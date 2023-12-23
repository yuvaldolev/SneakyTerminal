use glam::UVec2;

pub struct Arena {
    size: UVec2,
}

impl Arena {
    pub fn new(size: UVec2) -> Self {
        Self { size }
    }

    pub fn get_width(&self) -> u32 {
        self.size.x
    }

    pub fn get_height(&self) -> u32 {
        self.size.y
    }
}
