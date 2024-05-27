use html_escape::encode_safe;
use std::fmt::Display;

/// Trait for rendering something to a string
pub trait Render {
    /// Rendering a structure into a string
    fn render(&self) -> String;
}

impl<T: Display> Render for T {
    fn render(&self) -> String {
        encode_safe(&self.to_string()).to_string()
    }
}
