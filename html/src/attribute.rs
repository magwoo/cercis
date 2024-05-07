use html_escape::encode_quoted_attribute;

#[derive(Clone)]
pub struct Attribute {
    pub name: String,
    pub value: Option<String>,
    pub is_raw: bool,
}

impl Attribute {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: None,
            is_raw: false,
        }
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());

        self
    }

    pub fn raw(mut self) -> Self {
        self.is_raw = true;

        self
    }

    pub fn render(&self) -> String {
        match self.value.as_ref() {
            Some(value) => match self.is_raw {
                true => format!(" {}='{}'", self.name, value),
                false => format!(" {}='{}'", self.name, encode_quoted_attribute(value)),
            },
            None => format!(" {}", self.name),
        }
    }
}
