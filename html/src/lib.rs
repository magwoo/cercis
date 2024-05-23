use std::cell::RefCell;
use std::rc::Rc;

use crate::prelude::*;

pub mod attribute;
pub mod component;
pub mod content;
pub mod element;
pub mod prelude;
pub mod render;

pub type Element<'a> = VBody<'a>;

#[derive(Default, Clone)]
pub struct VBody<'a>(Rc<RefCell<Vec<Box<dyn Render + 'a>>>>);

impl<'a> VBody<'a> {
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(Vec::new())))
    }

    pub fn child(self, child: impl Render + 'a) -> Self {
        self.0.borrow_mut().push(Box::new(child));

        self
    }
}

impl<'a> Render for VBody<'a> {
    fn render(&self) -> String {
        let mut body = String::new();

        for child in self.0.borrow().iter() {
            body.push_str(&child.render())
        }

        body
    }
}

impl<'a> Render for &VBody<'a> {
    fn render(&self) -> String {
        (*self).render()
    }
}
