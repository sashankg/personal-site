use leptos::*;

use crate::routes::{home::HomePage, movies::MoviesPage, projects::ProjectsPage};

#[component]
pub fn App(path: String, db_pool: sqlx::SqlitePool) -> impl IntoView {
    provide_context(store_value(db_pool));
    view! {
        <body class="bg-background h-screen bg-background text-text p-8">
            <Router route={path}>
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

#[component]
pub fn Router(route: String, children: Children) -> impl IntoView {
    provide_context(store_value(route));
    children()
}

#[component]
pub fn Route(path: &'static str, children: ChildrenFn) -> impl IntoView {
    let context = use_context::<StoredValue<String>>();
    let matches_path = context.map_or(false, |cx| {
        cx.with_value(|current_path| current_path == path)
    });
    view! {
        <Show when={move || matches_path}>
            {children()}
        </Show>
    }
}
