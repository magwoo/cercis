![cercis-preview](../.github/assets/cercis-preview.png)

Template engine for HTML in Rust

```sh
cargo add cercis
```

# Using examples

## Formatting

Format the data into a string as in ```format!()``` macro

> all data is transferred to the template by reference

```rust
use cercis::prelude::*;

fn main() {
  let name = "Boris";
  
  let page = rsx!(h1 { "Hello {name}!" });

  // output: <h1>Hello Boris!</h1>
  println!("{}", page.render())
}
```

## Attributes

Attributes are written before the tag content as ```tag: value```

```rust
use cercis::prelude::*;

fn main() {
  let text_id = "paragraph";
  let text = "Lorem ipsum";

  let page = rsx!(div {
    class: "container",

    h1 { "Hello World!" }
    p {
      id: "{text_id}",
    
      "{text}"
    }
  });

  // output: <div class='container'><h1>Hello World!</h1><p id='paragraph'>Lorem ipsum</p></div>
  println!("{}", page.render())
}
```

## If branching

Using the usual Rust if syntax, you can create branches

> at the moment, ```else if``` ```else``` is not supported

```rust
use cercis::prelude::*;

fn main() {
  let num = 8;

  let page = rsx!(div {
    if num == 9 {
      span { "Num is 9" }
    }
    if num == 8 {
      span { "Num is 8" }
    }
  });

  // output: <div><span>Num is 8</span></div>
  println!("{}", page.render())
}
```

## Loops

Using the usual Rust ```for in``` you can create loops

```rust
use cercis::prelude::*;

fn main() {
  let names = vec!["Boris", "Polina", "Igor"];

  let page = rsx!(ol {
    for name in names {
      li { "{name}" }
    }
  });

  // output: <ol><li>Boris</li><li>Polina</li><li>Igor</li></ol>
  println!("{}", page.render())
}
```

## Component with params

Parameters are declared as normal function parameters

```rust
use cercis::prelude::*;

fn main() {
  let text = "Lorem ipsum";

  let page = rsx!(div {
    MyComponent {
        text: text
    }
  });

  // output: <div><p>Lorem ipsum</p></div>
  println!("{}", page.render())
}

#[component]
fn MyComponent<'a>(text: &'a str) -> Element {
    rsx!(p { "{text}" })
}
```

## Component with option params

If the parameter is an option, then you can omit it when calling the component

```rust
use cercis::prelude::*;

fn main() {
  let text = "Lorem ipsum";

  let page = rsx!(div {
    MyComponent {}
    MyComponent {
        text: text
    }
  });

  // output: <div><p>empty</p><p>Lorem ipsum</p></div>
  println!("{}", page.render())
}

#[component]
fn MyComponent<'a>(text: Option<&'a str>) -> Element {
    let text = text.unwrap_or("empty");

    rsx!(p { "{text}" })
}
```

## Component with children

the component can accept elements ```example: Element``` and children if you name the variable ```children: Element```

> all ```Element``` types are optional

```rust
use cercis::prelude::*;

fn main() {
    let text = "Lorem ipsum";

    let page = rsx!(div {
      MyComponent { span { "children" } }
      MyComponent {
          text: rsx!(p { "{text}" }),

          span { "children" }
      }
      MyComponent { text: rsx!(p { "my text" }) }
    });

    /* output(formatted):
    <div>
        <div class='container'>
            <div></div>
            <span>children</span>
        </div>
        <div class='container'>
            <div><p>Lorem ipsum</p></div>
            <span>children</span>
        </div>
        <div class='container'>
            <div><p>my text</p></div>
        </div>
    </div>
    */
    println!("{}", page.render())
}

#[component]
fn MyComponent(text: Element, children: Element) -> Element {
    rsx!(div {
        class: "container",

        div { text }
        children
    })
}
```

## Full page

```rust
use cercis::prelude::*;

fn main() {
    // declarate people names
    let names = vec!["Boris", "Polina", "Igor", "Nikita"];

    // create peoples ordered list
    let peoples = rsx!(ol {
        for name in names {
            li { "{name}" }
        }
    });

    // create full page using our Page component
    let page = rsx!(Page {
        title: "Cercis",

        h1 { "Peoples:" }
        // insert element from peoples variable
        peoples
    });

    // render our page to string
    println!("{}", page.render())
}

#[component]
pub fn Page<'a>(title: Option<&'a str>, head: Element, children: Element) -> Element {
    const META_CONTENT: &str = "witdh=device-width, initial-scale=1.0";

    rsx!(
        // turn into: <!DOCTYPE html>
        doctype {}
        html {
            head {
                meta { charset: "UTF-8" }
                meta {
                    name: "viewport",
                    // interpolate META_CONTENT const into value like format!() macro
                    content: "{META_CONTENT}",
                }
                head
                // declarate title if exists
                if let Some(title) = title {
                    title { "{title}" }
                }
            }
            body { children }
        }

    )
}
```

> If you have any problems [create issue](https://github.com/magwoo/cercis/issues)
