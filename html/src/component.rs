use std::rc::Rc;

pub trait Component {
    fn render(&self) -> String;
}

#[derive(Clone)]
pub struct VComponent(Rc<dyn Component>);

impl VComponent {
    pub fn new(inner: Rc<dyn Component>) -> Self {
        Self(inner)
    }

    pub fn render(&self) -> String {
        self.0.render()
    }
}

// pub trait RenderComponent: 'static {
//     fn render<'a, T>(&self, props: Props<'a, T>) -> VBody;
// }

// pub struct Props<'a, T>(&'a T);

// impl<'a, T> Props<'a, T> {
//     pub fn new(props: &'a T) -> Self {
//         Self(props)
//     }
// }

// #[derive(Clone)]
// pub struct Component<'a, T>(Rc<ComponentInner<'a, T>>);

// impl<'a, T> Component<'a, T> {
//     pub fn new(func: impl RenderComponent, props: &'a T) -> Self {
//         Self(Rc::new(ComponentInner::new(func, props)))
//     }

//     pub fn render(&self) -> String {
//         self.0.render()
//     }
// }

// struct ComponentInner<'a, T> {
//     func: Box<dyn RenderComponent>,
//     props: Props<'a, T>,
// }

// impl<'a, T> ComponentInner<'a, T> {
//     fn new(func: impl RenderComponent, props: &'a T) -> Self {
//         Self {
//             func: Box::new(func),
//             props: Props::new(props),
//         }
//     }

//     fn render(&self) -> String {
//         self.func.render(self.props).render()
//     }
// }

// impl<F> RenderComponent for F
// where
//     F: Fn(Props<'a, T>) -> VBody + 'static,
// {
//     fn render<'a, T>(&self, props: Props<'a, T>) -> VBody {
//         self(props)
//     }
// }
