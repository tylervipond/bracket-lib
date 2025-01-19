use bevy::color::Srgba;

use super::constants::{BLACK, WHITE};

#[derive(Clone, Debug)]

pub struct ColorPair {
    pub fg: Srgba,
    pub bg: Srgba,
}

impl ColorPair {
    pub fn new(fg: Srgba, bg: Srgba) -> Self {
        Self { fg, bg }
    }
}

impl Default for ColorPair {
    fn default() -> Self {
        Self::new(WHITE, BLACK)
    }
}
