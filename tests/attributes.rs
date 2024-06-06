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
