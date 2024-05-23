use html_escape::encode_safe;
use std::fmt::Display;

pub trait Render {
    fn render(&self) -> String;
}

impl<T: Display> Render for T {
    fn render(&self) -> String {
        encode_safe(&self.to_string()).to_string()
    }
}
