use crate::prelude::*;

pub struct VComponent<'a>(Box<dyn Component + 'a>);

pub trait Component {
    fn render(&self) -> VBody;
}

impl<'a> VComponent<'a> {
    pub fn new(component: Box<dyn Component + 'a>) -> Self {
        Self(component)
    }
}

impl<'a> Render for VComponent<'a> {
    fn render(&self) -> String {
        self.0.render().render()
    }
}
