use html_escape::encode_safe;
use std::borrow::Cow;

use crate::prelude::*;

#[derive(Clone)]
pub struct VContent<'a> {
    content: Cow<'a, str>,
    is_raw: Option<bool>,
}

impl<'a> VContent<'a> {
    pub fn new(content: impl Into<Cow<'a, str>>) -> Self {
        Self {
            content: content.into(),
            is_raw: None,
        }
    }

    pub fn raw(mut self, value: bool) -> Self {
        self.is_raw = Some(value);

        self
    }
}

impl<'a> Render for VContent<'a> {
    fn render(&self) -> String {
        match self.is_raw {
            Some(val) if val => self.content.to_string(),
            _ => encode_safe(&self.content).to_string(),
        }
    }
}
