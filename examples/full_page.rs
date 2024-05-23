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
pub fn Page<'a>(title: Option<&'a str>, children: Element<'a>) -> Element {
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
                // declarate title if exists
                if let Some(title) = title {
                    title { "{title}" }
                }
            }
            body { children }
        }

    )
}
