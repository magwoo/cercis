use std::borrow::Cow;

use html_escape::encode_safe;

#[derive(Clone)]
pub struct VAttribute<'a> {
    name: Cow<'a, str>,
    value: Option<Cow<'a, str>>,
    is_raw: Option<bool>,
}

impl<'a> VAttribute<'a> {
    pub fn new(name: impl Into<Cow<'a, str>>) -> Self {
        Self {
            name: name.into(),
            value: None,
            is_raw: None,
        }
    }

    pub fn value(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.value = Some(value.into());

        self
    }

    pub fn raw(mut self, value: bool) -> Self {
        self.is_raw = Some(value);

        self
    }

    pub fn render(&self) -> String {
        match self.value.as_ref() {
            Some(value) => match self.is_raw {
                Some(val) if val => format!(" {}='{}'", self.name, value),
                _ => format!(" {}='{}'", self.name, encode_safe(&value)),
            },
            None => format!(" {}", self.name),
        }
    }
}
