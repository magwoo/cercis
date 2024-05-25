![cercis-preview](../.github/assets/cercis-preview.png)

Macro for [cercis](https://crates.io/crates/cercis) package

```sh
cargo add cercis
```

> Used only with the [cercis](https://crates.io/crates/cercis) package

# Using examples

For more examples, see [cercis](https://crates.io/crates/cercis)

## Empty component

> all data is transferred to the component by reference

```rust
use cercis::prelude::*;

fn main() {
  let page = rsx!(div {
    MyComponent {}
  });

  // output: <div><h1>my component</h1></div>
  println!("{}", page.render())
}

#[component]
fn MyComponent() -> Element {
    rsx!(h1 { "my component" })
}
```

## Params

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

## Option params

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

## Children

the component can accept elements ```example: Element<'a>``` and children if you name the variable ```children: Element<'a>```

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
fn MyComponent<'a>(text: Element<'a>, children: Element<'a>) -> Element {
    rsx!(div {
        class: "container",

        div { text }
        children
    })
}
```

> If you have any problems [create issue](https://github.com/magwoo/cercis/issues)
