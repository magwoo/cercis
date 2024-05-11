use cercis::prelude::*;
use rand::random;

#[test]
fn simple_tag() {
    let correct = "<h1>Hello World!</h1>";
    let render = rsx!(h1 { "Hello World!" }).render();

    assert_eq!(render, correct)
}

#[test]
fn complex_tag() {
    let correct = "<span id='hello' class='world' hello-world='true'>Content</span>";
    let render = rsx!(span {
        id: "hello",
        class: "world",
        hello_world: true,

        "Content"
    })
    .render();

    assert_eq!(render, correct)
}

#[test]
fn simple_single_tag() {
    let correct = "<link>";
    let render = rsx!(link { "unexpected content" }).render();

    assert_eq!(render, correct)
}

#[test]
fn doctype_tag() {
    let correct = "<!DOCTYPE html>";
    let render = rsx!(doctype {}).render();

    assert_eq!(render, correct)
}

#[test]
fn multiple_tags() {
    let correct = "<p>Tag 1</p><p>Tag 2</p>";
    let render = rsx!(p { "Tag 1" } p { "Tag 2" }).render();

    assert_eq!(render, correct)
}

#[test]
fn nested_tags() {
    let correct = "<div><span>span 1</span><span>span 2</span></div>";
    let render = rsx!(div {span { "span 1" } span { "span 2" }}).render();

    assert_eq!(render, correct)
}

#[test]
fn format_tag() {
    let num = random::<u8>();

    let correct = format!("<h1>Random num is {num}</h1>");
    let render = rsx!(h1 { "Random num is {num}" }).render();

    assert_eq!(render, correct)
}

#[test]
fn for_loop() {
    let mut nums = Vec::new();

    for _ in 0..10 {
        nums.push(random::<u8>());
    }

    let mut correct = String::from("<h1>Random nums:</h1>");

    for num in nums.iter() {
        correct.push_str(format!("<span>{num}</span>").as_str());
    }

    let render = rsx!(
        h1 { "Random nums:" }
        for num in nums {
            span { "{num}" }
        }
    )
    .render();

    assert_eq!(render, correct)
}
