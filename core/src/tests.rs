use crate::prelude::*;

#[test]
fn simple_tag() {
    let correct = "<h1>Hello World!</h1>";
    let render = rsx!(h1 { "Hello World!" }).render();

    assert_eq!(render, correct)
}

#[test]
fn multiple_tags() {
    let correct = "<p>Tag 1</p><p>Tag 2</p>";
    let render = rsx!(p { "Tag 1" } p { "Tag 2" }).render();

    assert_eq!(render, correct)
}

#[test]
fn nested_tag() {
    let correct = "<div><span>span 1</span><span>span 2</span></div>";
    let render = rsx!(div {span { "span 1" } span { "span 2" }}).render();

    assert_eq!(render, correct)
}
