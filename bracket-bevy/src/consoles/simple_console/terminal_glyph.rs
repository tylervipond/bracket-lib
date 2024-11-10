#[derive(Clone, Copy)]
pub struct TerminalGlyph {
    pub(crate) glyph: u16,
    pub(crate) foreground: [f32; 4],
    pub(crate) background: [f32; 4],
}

impl Default for TerminalGlyph {
    fn default() -> Self {
        Self {
            glyph: 32,
            foreground: [1.0, 1.0, 1.0, 1.0],
            background: [0.0, 0.0, 0.0, 1.0],
        }
    }
}
