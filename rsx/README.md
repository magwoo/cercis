![cercis-preview](../.github/assets/cercis-preview.png)

Macro for [cercis](https://crates.io/crates/cercis) package

```sh
cargo add cercis
```

> Used only with the [cercis](https://crates.io/crates/cercis) package

# Using examples

For more examples, see [cercis](https://crates.io/crates/cercis)

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

> If you have any problems [create issue](https://github.com/magwoo/cercis/issues)
