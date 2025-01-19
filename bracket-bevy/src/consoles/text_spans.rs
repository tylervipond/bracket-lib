use crate::{
    color::constants::{BLACK, WHITE},
    BracketContext,
};
use bevy::color::Srgba;

#[derive(Debug)]
pub struct ColoredTextSpans {
    pub length: usize,
    pub spans: Vec<(Srgba, String)>,
}

impl ColoredTextSpans {
    pub fn new(context: &BracketContext, text: &str) -> Self {
        let mut result = Self {
            length: 0,
            spans: Vec::new(),
        };
        let mut color_stack: Vec<&Srgba> = Vec::new();

        for color_span in text.split("#[") {
            if color_span.is_empty() {
                continue;
            }
            let mut col_text = color_span.splitn(2, ']');
            let col_name = col_text.next().unwrap();
            if let Some(text_span) = col_text.next() {
                if !col_name.is_empty() {
                    color_stack.push(context.get_named_color(col_name).unwrap_or(&BLACK));
                } else {
                    color_stack.pop();
                }
                result.spans.push((
                    **color_stack.last().unwrap_or(&&WHITE),
                    text_span.to_string(),
                ));
                result.length += text_span.chars().count();
            }
        }

        result
    }
}
