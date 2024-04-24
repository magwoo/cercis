use std::any::Any;
use std::rc::Rc;

use crate::VBody;

pub trait RenderComponent: 'static {
    fn render(&self, props: &dyn Any) -> VBody;
}

#[derive(Clone)]
pub struct Component(Rc<ComponentInner>);

impl Component {
    pub fn new(func: impl RenderComponent, props: Box<dyn Any>) -> Self {
        Self(Rc::new(ComponentInner::new(func, props)))
    }

    pub fn render(&self) -> String {
        self.0.render()
    }
}

struct ComponentInner {
    func: Box<dyn RenderComponent>,
    props: Box<dyn Any>,
}

impl ComponentInner {
    fn new(func: impl RenderComponent, props: Box<dyn Any>) -> Self {
        Self {
            func: Box::new(func),
            props,
        }
    }

    fn render(&self) -> String {
        self.func.render(self.props.as_ref()).render()
    }
}

impl<F> RenderComponent for F
where
    F: Fn(&dyn Any) -> VBody + 'static,
{
    fn render(&self, props: &dyn Any) -> VBody {
        self(props)
    }
}
