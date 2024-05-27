use cercis::prelude::*;

#[test]
fn raw_static_str() {
    let correct = "<section><p>lorem</p></section>";
    let render = rsx!(section { r"<p>lorem</p>" }).render();

    assert_eq!(render, correct)
}

#[test]
fn raw_fmt_str() {
    let lorem = "lorem";

    let correct = "<section><p>lorem</p></section>";
    let render = rsx!(section { r"<p>{lorem}</p>"}).render();

    assert_eq!(render, correct)
}

#[test]
fn raw_var() {
    let lorem = "<p>lorem</p>";

    let correct = "<section><p>lorem</p></section>";
    let render = rsx!(section { r"{lorem}"}).render();

    assert_eq!(render, correct)
}

#[test]
fn raw_attribute_static_str() {
    let correct = "<section class='>my-class<'></section>";
    let render = rsx!(section {
        class: r">my-class<"
    })
    .render();

    assert_eq!(render, correct)
}

#[test]
fn raw_attribute_fmt_str() {
    let my_class = "my-class";

    let correct = "<section class='>my-class<'></section>";
    let render = rsx!(section {
        class: r">{my_class}<"
    })
    .render();

    assert_eq!(render, correct)
}

#[test]
fn raw_attribute_var() {
    let my_class = ">my-class<";

    let correct = "<section class='>my-class<'></section>";
    let render = rsx!(section {
        class: r"{my_class}"
    })
    .render();

    assert_eq!(render, correct)
}
