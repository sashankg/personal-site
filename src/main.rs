mod app;
mod components;
mod islands;
mod router;
mod routes;

use crate::app::*;
use leptos::*;

const ROUTES: [&str; 3] = ["/", "/projects", "/movies"];

#[cfg(feature = "ssg")]
fn main() {
    use sqlx::SqlitePool;
    use std::fs;

    let template = mustache::compile_path("index.mustache").unwrap();

    let rt = tokio::runtime::Runtime::new().unwrap();

    // Construct a local task set that can run `!Send` futures.
    let local = tokio::task::LocalSet::new();

    for path in ROUTES.iter() {
        let body = local.block_on(&rt, async {
            let db_pool = SqlitePool::connect("data.db").await.unwrap();
            leptos::ssr::render_to_string_async(|| {
                App(AppProps {
                    path: path.to_string(),
                    base_url: "/personal-site/".to_string(),
                    db_pool,
                })
                .into_view()
            })
            .await
        });
        let path = if *path == "/" { "index" } else { path };
        let data = mustache::MapBuilder::new()
            .insert_str("baseUrl", "/personal-site/")
            .insert_str("body", body)
            .build();
        let mut file = fs::File::create(format!("target/site/pkg/{}.html", path)).unwrap();
        match template.render_data(&mut file, &data) {
            Ok(_) => println!("Successfully rendered {}", path),
            Err(e) => println!("Failed to render {}: {}", path, e),
        }
    }
}

#[cfg(feature = "dev")]
fn main() {
    use rouille::Response;
    use sqlx::SqlitePool;
    rouille::start_server("127.0.0.1:3000", move |request| {
        if request.url() == "/" || request.url() == "/projects" || request.url() == "/movies" {
            let template = mustache::compile_path("index.mustache").unwrap();

            let rt = tokio::runtime::Runtime::new().unwrap();

            // Construct a local task set that can run `!Send` futures.
            let local = tokio::task::LocalSet::new();

            let path = request.url();

            let body = local.block_on(&rt, async {
                let db_pool = SqlitePool::connect("data.db").await.unwrap();
                leptos::ssr::render_to_string_async(|| {
                    App(AppProps {
                        path,
                        db_pool,
                        base_url: "".to_string(),
                    })
                    .into_view()
                })
                .await
            });
            println!("Rendered {}", body);
            let data = mustache::MapBuilder::new()
                .insert_str("baseUrl", "/target/site/pkg/")
                .insert_str("body", body)
                .build();
            return Response::html(template.render_data_to_string(&data).unwrap());
        }
        rouille::match_assets(request, ".")
    });
}
