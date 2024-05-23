use std::borrow::Cow;

use crate::prelude::*;

pub struct VElement<'a> {
    name: Cow<'a, str>,
    attrs: Vec<VAttribute<'a>>,
    children: Vec<Box<dyn Render + 'a>>,
    is_single: Option<bool>,
}

impl<'a> VElement<'a> {
    pub fn new(name: impl Into<Cow<'a, str>>) -> Self {
        Self {
            name: name.into(),
            attrs: Vec::new(),
            children: Vec::new(),
            is_single: None,
        }
    }

    pub fn child(mut self, child: impl Render + 'a) -> Self {
        self.children.push(Box::new(child));

        self
    }

    pub fn attr(mut self, attr: impl Into<VAttribute<'a>>) -> Self {
        self.attrs.push(attr.into());

        self
    }

    pub fn single(mut self, value: bool) -> Self {
        self.is_single = Some(value);

        self
    }
}

impl<'a> Render for VElement<'a> {
    fn render(&self) -> String {
        let mut attrs = String::new();
        let mut children = String::new();

        for attr in self.attrs.iter() {
            attrs.push_str(&attr.render())
        }

        for child in self.children.iter() {
            children.push_str(&child.render())
        }

        match self.is_single {
            Some(true) => format!("<{0}{1}>", self.name, attrs),
            _ => format!("<{0}{1}>{2}</{0}>", self.name, attrs, children),
        }
    }
}
