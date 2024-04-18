pub mod macros {
    pub use cercis_rsx::rsx;
}

pub mod html {
    pub use cercis_html::prelude::*;
}

pub mod prelude {
    pub use crate::macros::rsx;
}
