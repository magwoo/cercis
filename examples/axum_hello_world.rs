use axum::response::Html;
use axum::routing::get;
use axum::Router;
use cercis::prelude::*;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let router = Router::new().route("/", get(index));

    let listener = TcpListener::bind("0.0.0.0:8080").await?;

    println!("page avaliable at http://127.0.0.1:8080");
    axum::serve(listener, router).await?;

    Ok(())
}

async fn index() -> Html<String> {
    let page = rsx!(Page {
        title: "Hello cercis!",

        h1 { "Hello cercis!" }
    });

    Html(page.render())
}

#[component]
pub fn Page<'a>(title: Option<&'a str>, children: Element<'a>) -> Element {
    const META_CONTENT: &str = "witdh=device-width, initial-scale=1.0";

    rsx!(
        doctype {}
        html {
            head {
                meta { charset: "UTF-8" }
                meta {
                    name: "viewport",
                    content: "{META_CONTENT}",
                }
                if let Some(title) = title {
                    title { "{title}" }
                }
            }
            body { children }
        }

    )
}
