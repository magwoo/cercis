use smallvec::SmallVec;
use std::borrow::Cow;

use crate::prelude::*;

const CHLD_DEFAULT: usize = 8;
const ATTR_DEFAULT: usize = 4;

pub struct VElement<'a, const CHLD: usize = CHLD_DEFAULT, const ATTR: usize = ATTR_DEFAULT> {
    name: Cow<'a, str>,
    attrs: SmallVec<[VAttribute<'a>; ATTR]>,
    children: SmallVec<[Box<dyn Render + 'a>; CHLD]>,
    is_single: Option<bool>,
}

impl<'a> VElement<'a> {
    pub fn new(name: impl Into<Cow<'a, str>>) -> Self {
        Self {
            name: name.into(),
            attrs: SmallVec::new(),
            children: SmallVec::new(),
            is_single: None,
        }
    }

    pub fn new_sized<const C: usize, const A: usize>(
        name: impl Into<Cow<'a, str>>,
    ) -> VElement<'a, C, A> {
        VElement {
            name: name.into(),
            attrs: SmallVec::new(),
            children: SmallVec::new(),
            is_single: None,
        }
    }
}

impl<'a, const CHLD: usize, const ATTR: usize> VElement<'a, CHLD, ATTR> {
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

    pub fn get_children_presize(&self) -> usize {
        CHLD
    }

    pub fn get_attributes_presize(&self) -> usize {
        ATTR
    }

    pub fn is_children_spilled(&self) -> bool {
        self.children.spilled()
    }

    pub fn is_attributes_spilled(&self) -> bool {
        self.attrs.spilled()
    }
}

impl<'a, const CHLD: usize, const ATTR: usize> Render for VElement<'a, CHLD, ATTR> {
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
