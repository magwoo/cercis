/*!
# Template html at rust code

Read more about macros at [`crate::macros`]

# Examples

## Render template to string

```
use cercis::prelude::*;

let age = 19;
let page = rsx!(h1 { "Your age is {age}" });

let output = "<h1>Your age is 19</h1>";

assert_eq!(page.render(), output)
```

## For loop and if

```
use cercis::prelude::*;

let nums = vec![1, 2, 3];
let page = rsx!(ul {
    for num in nums {
        li { "{num}" }
        if num == 2 {
            span { "num is 2" }
        }
    }
});

let output = "<ul><li>1</li><li>2</li><span>num is 2</span><li>3</li></ul>";

assert_eq!(page.render(), output)
```

## Components

```
use cercis::prelude::*;

let page = rsx!(div {
    Button { text: "prev" }
    Button { text: "next" }
});

let output = "<div><button class='component-button'>prev</button><button class='component-button'>next</button></div>";

assert_eq!(page.render(), output);

#[component]
fn Button<'a>(text: &'a str, children: Element) -> Element {
    rsx!(button {
        class: "component-button",

        "{text}"
        children
    })
}
```
*/

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
    pub use cercis_html::VBody as Element;
    pub use typed_builder;
}
