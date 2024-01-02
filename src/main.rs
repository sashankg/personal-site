mod app;
mod components;
mod islands;
mod router;
mod routes;

use crate::app::*;
use crate::router::generate_route_list;
use leptos::*;
use sqlx::SqlitePool;

#[cfg(feature = "ssg")]
#[tokio::main]
async fn main() {
    use std::fs;

    let template = mustache::compile_path("index.mustache").unwrap();

    let db_pool = SqlitePool::connect("data.db").await.unwrap();

    let db_pool_clone = db_pool.clone();
    let route_list = generate_route_list(move || {
        App(AppProps {
            path: "".to_string(),
            db_pool: db_pool_clone,
            base_url: "".to_string(),
        })
    });

    let local = tokio::task::LocalSet::new();

    for path in route_list {
        let db_pool = db_pool.clone();
        let path_clone = path.clone();
        let body = local
            .run_until(leptos::ssr::render_to_string_async(|| {
                App(AppProps {
                    path: path_clone,
                    base_url: "/personal-site/".to_string(),
                    db_pool,
                })
                .into_view()
            }))
            .await;
        let path = if path == "/" {
            "index".to_string()
        } else {
            path
        };
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
#[tokio::main]
async fn main() {
    use rouille::Response;

    let db_pool = SqlitePool::connect("data.db").await.unwrap();

    let db_pool_clone = db_pool.clone();
    let route_list = generate_route_list(move || {
        App(AppProps {
            path: "".to_string(),
            db_pool: db_pool_clone,
            base_url: "".to_string(),
        })
    });

    rouille::start_server("127.0.0.1:3000", move |request| {
        let template = mustache::compile_path("index.mustache").unwrap();
        let db_pool = db_pool.clone();
        if route_list.contains(&request.url()) {
            let path = request.url();

            let rt = tokio::runtime::Runtime::new().unwrap();

            // Construct a local task set that can run `!Send` futures.
            let local = tokio::task::LocalSet::new();
            let body = local.block_on(&rt, async {
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
            let data = mustache::MapBuilder::new()
                .insert_str("baseUrl", "/target/site/pkg/")
                .insert_str("body", body)
                .build();
            return Response::html(template.render_data_to_string(&data).unwrap());
        }
        rouille::match_assets(request, ".")
    });
}
