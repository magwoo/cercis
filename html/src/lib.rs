use smallvec::SmallVec;
use std::cell::RefCell;
use std::rc::Rc;

use crate::prelude::*;

pub mod attribute;
pub mod component;
pub mod content;
pub mod element;
pub mod prelude;
pub mod render;

const CHLD_DEFAULT: usize = 8;

/// Type alias for [`VBody`] struct as ```VBody<'a>```
pub type Element<'a, const CHLD: usize = CHLD_DEFAULT> = VBody<'a, CHLD>;

type VBodyInner<'a, const CHLD: usize> = Rc<RefCell<SmallVec<[Box<dyn Render + 'a>; CHLD]>>>;

/// Container for any elements which implements [`Render`] trait
#[derive(Default, Clone)]
pub struct VBody<'a, const CHLD: usize = CHLD_DEFAULT>(VBodyInner<'a, CHLD>);

impl<'a> VBody<'a> {
    /// Create new container with empty children
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(SmallVec::new())))
    }

    pub fn new_sized<const C: usize>() -> VBody<'a, C> {
        VBody(Rc::new(RefCell::new(SmallVec::new())))
    }
}

impl<'a, const CHLD: usize> VBody<'a, CHLD> {
    /// Add child into container (The elements must implement trait [`Render`])
    pub fn child(self, child: impl Render + 'a) -> Self {
        self.0.borrow_mut().push(Box::new(child));

        self
    }
}

impl<const CHLD: usize> Render for VBody<'_, CHLD> {
    fn render(&self) -> String {
        let mut body = String::new();

        for child in self.0.borrow().iter() {
            body.push_str(&child.render())
        }

        body
    }
}

impl<const CHLD: usize> Render for &VBody<'_, CHLD> {
    fn render(&self) -> String {
        (*self).render()
    }
}
