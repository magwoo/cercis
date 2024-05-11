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

/// Using macros ```rsx!()``` and ```#[component]```, you can create HTML pages directly in the Rust language
///
/// # Examples
///
/// ## Full page
///
/// ```
/// use cercis::prelude::*;
///
/// // declarate people names
/// let names = vec!["Boris", "Polina", "Igor", "Nikita"];
///
/// // create peoples ordered list
/// let peoples = rsx!(ol {
///     for name in names {
///         li { "{name}" }
///     }
/// });
///
/// // create full page using our Page component
/// let page = rsx!(Page {
///     title: "Cercis",
///
///     h1 { "Peoples:" }
///     // insert element from peoples variable
///     peoples
/// });
///
/// // render our page to string
/// println!("{}", page.render());
///
/// #[component]
/// pub fn Page<'a>(title: Option<&'a str>, children: Element) -> Element {
///     const META_CONTENT: &str = "witdh=device-width, initial-scale=1.0";
///
///     rsx!(
///         // turn into: <!DOCTYPE html>
///         doctype {}
///         html {
///             head {
///                 meta { charset: "UTF-8" }
///                 meta {
///                     name: "viewport",
///                     // interpolate META_CONTENT const into value like format!() macro
///                     content: "{META_CONTENT}",
///                 }
///                 // declarate title if exists
///                 if let Some(title) = title {
///                     title { "{title}" }
///                 }
///             }
///             body { children }
///         }
///
///     )
/// }
/// ```
pub mod macros {
    pub use cercis_component::component;
    pub use cercis_rsx::rsx;
}

/// Re-export template build types from [`cercis-html`] for ```rsx!``` macro
pub mod html {
    pub use cercis_html::prelude::*;
}

/// Re-export packages for ```rsx!``` and ```#[component]``` macro
pub mod system {
    pub use cercis_html;
    pub use typed_builder;
}

/// Package prelude
pub mod prelude {
    pub use crate::macros::*;
    pub use cercis_html::Element;
}
