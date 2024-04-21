pub use typed_builder;

pub mod macros {
    pub use cercis_component::component;
    pub use cercis_rsx::rsx;
}

pub mod html {
    pub use cercis_html::prelude::*;
}

pub mod prelude {
    pub use crate::macros::*;
    pub use cercis_html;
    pub use typed_builder;
}

#[cfg(test)]
mod rsx_tests {
    use super::prelude::*;

    #[test]
    fn simple_tag() {
        let correct = "<h1>Hello World!</h1>";
        let render = rsx!(h1 { "Hello World!" }).render();

        assert_eq!(render, correct)
    }

    #[test]
    fn nested_tag() {
        let correct = "<div><h1>Hello</h1><h2>World!</h2></div>";
        let render = rsx!(div {
            h1 { "Hello" }
            h2 { "World!" }
        })
        .render();

        assert_eq!(render, correct)
    }
}
