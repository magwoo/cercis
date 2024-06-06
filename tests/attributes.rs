use cercis::prelude::*;

#[test]
fn simple_attr() {
    let correct = "<div class='container'></div>";
    let render = rsx!(div { class: "container" }).render();

    assert_eq!(render, correct)
}

#[test]
fn multiple_attrs() {
    let correct = "<div id='main-container' class='container'></div>";
    let render = rsx!(div {
        id: "main-container",
        class: "container"
    })
    .render();

    assert_eq!(render, correct)
}

#[test]
fn single_tag_attr() {
    let correct = "<input placeholder='username'>";
    let render = rsx!(input {
        placeholder: "username"
    })
    .render();

    assert_eq!(render, correct)
}

#[test]
fn raw_attr_name() {
    let correct = "<input type='text'>";
    let render = rsx!(input { r#type: "text" }).render();

    assert_eq!(render, correct)
}

#[test]
fn single_tag_multiple_attr() {
    let correct = "<input placeholder='some number' type='number'>";
    let render = rsx!(input {
        placeholder: "some number",
        r#type: "number"
    })
    .render();

    assert_eq!(render, correct)
}

#[test]
fn fmt_attr() {
    let class = "container";

    let correct = "<main class='container'></main>";
    let render = rsx!(main { class: "{class}" }).render();

    assert_eq!(render, correct)
}

#[test]
fn display_data_attr() {
    let correct = "<section class='true'></section>";
    let render = rsx!(section { class: true }).render();

    assert_eq!(render, correct)
}
