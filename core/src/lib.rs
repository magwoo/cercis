pub use typed_builder;

#[cfg(test)]
mod tests;

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
