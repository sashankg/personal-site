use leptos::*;

use crate::router::*;
use crate::routes::{home::HomePage, movies::MoviesPage, projects::ProjectsPage};

#[component]
pub fn App(
    path: String,
    #[prop(optional)] base_url: String,
    db_pool: sqlx::SqlitePool,
) -> impl IntoView {
    provide_context(store_value(db_pool));
    view! {
        <body class="bg-background h-screen bg-background text-text p-8">
            <Router route={path} base_url={base_url}>
                <Route path="/" >
                    <HomePage />
                </Route>
                <Route path="/projects">
                    <ProjectsPage />
                </Route>
                <Route path="/movies">
                    <MoviesPage />
                </Route>
            </Router>
        </body>
    }
}
