mod app;
mod components;
mod islands;
mod routes;

use crate::app::*;
use leptos::*;
use std::fs;

const ROUTES: [&str; 3] = ["/", "/projects", "/movies"];

#[cfg(feature = "ssg")]
fn main() {
    use sqlx::SqlitePool;
    let template = fs::read_to_string("index.html").expect("Unable to read index.html");

    let rt = tokio::runtime::Runtime::new().unwrap();

    // Construct a local task set that can run `!Send` futures.
    let local = tokio::task::LocalSet::new();

    for path in ROUTES.iter() {
        let body = local.block_on(&rt, async {
            let db_pool = SqlitePool::connect("data.db").await.unwrap();
            leptos::ssr::render_to_string_async(|| {
                App(AppProps {
                    path: path.to_string(),
                    db_pool,
                })
                .into_view()
            })
            .await
        });
        let path = if *path == "/" { "index" } else { path };
        match fs::write(
            format!("target/site/pkg/{}.html", path),
            template.replace("{body}", &body),
        ) {
            Ok(_) => println!("{} written", path),
            Err(e) => println!("Error writing {}: {}", path, e),
        }
    }
}

#[cfg(feature = "dev")]
fn main() {
    use rouille::Response;
    use sqlx::SqlitePool;
    rouille::start_server("127.0.0.1:3000", move |request| {
        if request.url() == "/" || request.url() == "/projects" || request.url() == "/movies" {
            let template = fs::read_to_string("index.html").expect("Unable to read index.html");

            let rt = tokio::runtime::Runtime::new().unwrap();

            // Construct a local task set that can run `!Send` futures.
            let local = tokio::task::LocalSet::new();

            let path = request.url();

            let body = local.block_on(&rt, async {
                let db_pool = SqlitePool::connect("data.db").await.unwrap();
                leptos::ssr::render_to_string_async(|| App(AppProps { path, db_pool }).into_view())
                    .await
            });

            return Response::html(template.replace("{body}", &body));
        }
        rouille::match_assets(request, ".")
    });
}

// #[cfg(feature = "hydrate")]
// fn main() {
//     println!("Hello, world!");
// }
