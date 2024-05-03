![cercis-preview](./.github/assets/cercis-preview.png)

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
        text: "{text}"
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
        text: "{text}"
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

> If you have any problems [create issue](https://github.com/magwoo/cercis/issues)
