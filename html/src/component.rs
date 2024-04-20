use std::any::Any;

use crate::VBody;

pub trait RenderComponent: 'static {
    fn render(&self, props: Box<dyn Any>) -> VBody;
}

pub trait ComponentProps: 'static {}

impl ComponentProps for i32 {}

pub struct Component {
    func: Box<dyn RenderComponent>,
    props: Box<dyn Any>,
}

impl Component {
    pub fn new(func: impl RenderComponent, props: Box<dyn Any>) -> Self {
        Self {
            func: Box::new(func),
            props,
        }
    }

    pub fn render(self) -> String {
        self.func.render(self.props).render()
    }
}

impl<F> RenderComponent for F
where
    F: Fn(Box<dyn Any>) -> VBody + 'static,
{
    fn render(&self, props: Box<dyn Any>) -> VBody {
        self(props)
    }
}
