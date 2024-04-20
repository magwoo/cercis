use std::sync::Mutex;

use crate::VBody;

pub trait RenderComponent: 'static {
    fn render(&self) -> VBody;
}

pub struct Component(Mutex<Box<dyn RenderComponent>>);

impl Component {
    pub fn new(func: impl RenderComponent) -> Self {
        Self(Mutex::new(Box::new(func)))
    }

    pub fn render(self) -> String {
        self.0.lock().unwrap().render().render()
    }
}

impl<F: Fn() -> VBody + 'static> RenderComponent for F {
    fn render(&self) -> VBody {
        self()
    }
}
