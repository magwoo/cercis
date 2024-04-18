use crate::attribute::Attribute;

pub mod attribute;
pub mod builder;
pub mod component;
pub mod prelude;

pub struct VBody(Vec<VNode>);

impl VBody {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn node(mut self, node: VNode) -> Self {
        self.0.push(node);

        self
    }

    pub fn merge(mut self, component: Self) -> Self {
        self.insert(component);

        self
    }

    pub fn insert(&mut self, component: Self) {
        for node in component.0.into_iter() {
            self.0.push(node);
        }
    }

    pub fn render(self) -> String {
        let mut result = String::new();

        for node in self.0 {
            result.push_str(node.render().as_str());
        }

        result
    }
}

impl Default for VBody {
    fn default() -> Self {
        Self::new()
    }
}

pub enum VNode {
    Element(VElement),
    Content(String),
}

impl VNode {
    pub fn element(element: impl Into<VElement>) -> Self {
        Self::Element(element.into())
    }

    pub fn content(text: impl Into<String>) -> Self {
        Self::Content(text.into())
    }

    pub fn render(self) -> String {
        match self {
            Self::Element(element) => element.render(),
            Self::Content(content) => content,
        }
    }
}

pub struct Component {
    pub func: Box<dyn Fn() -> VBody>,
}

pub struct VElement {
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub children: Vec<VNode>,
}

impl VElement {
    pub fn new(element: impl Into<String>) -> Self {
        Self {
            name: element.into(),
            attributes: Vec::new(),
            children: Vec::new(),
        }
    }

    pub fn attr(mut self, attr: impl Into<Attribute>) -> Self {
        self.attributes.push(attr.into());

        self
    }

    pub fn child(mut self, child: impl Into<VNode>) -> Self {
        self.children.push(child.into());

        self
    }

    pub fn merge(mut self, component: VBody) -> Self {
        for node in component.0.into_iter() {
            self.children.push(node);
        }

        self
    }

    pub fn node(self, child: impl Into<VNode>) -> Self {
        self.child(child)
    }

    pub fn render(self) -> String {
        let mut attrs = String::new();

        for attr in self.attributes {
            attrs.push_str(attr.render().as_str())
        }

        let mut content = String::new();

        for child in self.children {
            content.push_str(child.render().as_str());
        }

        format!("<{0}{1}>{2}</{0}>", self.name, attrs, content)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn empty_div() {
//         let page = "<div></div>";
//         let element = Element::new("div");

//         assert_eq!(page, element.render())
//     }

//     #[test]
//     fn div_with_text() {
//         let page = "<div>text content</div>";
//         let element = Element::new("div").text("text content");
//         assert_eq!(page, element.render())
//     }

//     #[test]
//     fn div_with_text_and_attrs() {
//         let page = "<div id='test-id' class>text content</div>";
//         let element = Element::new("div")
//             .attr("id", "test-id")
//             .empty_attr("class")
//             .text("text content");

//         assert_eq!(page, element.render())
//     }

//     #[test]
//     fn div_with_child() {
//         let page = "<div class='container'><p>paragraph</p></div>";
//         let element = Element::new("div")
//             .attr("class", "container")
//             .children(Element::new("p").text("paragraph"));

//         assert_eq!(page, element.render())
//     }
// }
