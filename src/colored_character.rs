pub static DEFAULT_COLOR: &[u8] = b"\x1B[0m";
pub static RED_COLOR: &[u8] = b"\x1B[31m";

#[derive(Clone, Copy)]
pub struct ColoredCharacter {
    character: char,
    color: &'static [u8],
}

impl ColoredCharacter {
    pub fn new_colored(character: char, color: &'static [u8]) -> Self {
        Self { character, color }
    }

    pub fn new_uncolored(character: char) -> Self {
        Self::new_colored(character, DEFAULT_COLOR)
    }

    pub fn get_character(&self) -> char {
        self.character
    }

    pub fn get_color(&self) -> &[u8] {
        self.color
    }
}
