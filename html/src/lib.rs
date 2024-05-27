use std::cell::RefCell;
use std::rc::Rc;

use crate::prelude::*;

pub mod attribute;
pub mod component;
pub mod content;
pub mod element;
pub mod prelude;
pub mod render;

/// Type alias for [`VBody`] struct as ```VBody<'a>```
pub type Element<'a> = VBody<'a>;

/// Container for any elements which implements [`Render`] trait
#[derive(Default, Clone)]
pub struct VBody<'a>(Rc<RefCell<Vec<Box<dyn Render + 'a>>>>);

impl<'a> VBody<'a> {
    /// Create new container with empty children
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(Vec::new())))
    }

    /// Add child into container (The elements must implement trait [`Render`])
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
