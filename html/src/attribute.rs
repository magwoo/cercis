use html_escape::encode_safe;

#[derive(Clone)]
pub struct Attribute {
    pub name: String,
    pub value: Option<String>,
}

impl Attribute {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: None,
        }
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());

        self
    }

    pub fn render(self) -> String {
        match self.value {
            Some(value) => format!(" {}='{}'", self.name, encode_safe(value.as_str())),
            None => format!(" {}", self.name),
        }
    }
}
