use leptos::*;
use serde::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Navigation {
    title: Option<String>,
    path: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct HomeContent {
    title: Option<String>,
    description: Option<String>,
}

#[component]
pub fn HomePage() -> impl IntoView {
    // let navigations: Resource<(), Vec<Navigation>> = create_resource(
    //     || (),
    //     |_| async move {
    //         let db_pool = use_context::<StoredValue<SqlitePool>>()
    //             .unwrap()
    //             .get_value();
    //         sqlx::query_as!(Navigation, "select title, path from navigations")
    //             .fetch_all(&db_pool)
    //             .await
    //             .unwrap();
    //     },
    // );
    view! {
        <Suspense fallback=|| { view! { <div>"Loading..."</div>}}>
            {move || { view! {
        <div class="flex flex-col gap-1 justify-center h-full">
        // {
        // navigations.get().map(|resource| {
        // resource.iter().map(|nav| {
        // view! {
        // <NavLink title={nav.title.clone().unwrap()} path={nav.path.clone().unwrap()}/>
        // }
        // }).collect::<Vec<_>>()
        // })
        // }
        <div class="flex-1 flex flex-row items-end">
            <h1 class="text-primary text-3xl font-medium font-display">Sashank Gogula</h1>
        </div>
        <div class="flex-1">
            <p class="max-w-prose">
                "Product Engineer at Glean interested in human-computer interaction, language technologies, education, and design"
            </p>
            <nav class="mt-4">
            <ul class="flex flex-col gap-2">
            <NavSection title="Portfolio">
                <NavLink title="Projects".to_string()/>
                <NavLink title="Gallery".to_string()/>
            </NavSection>
            <NavSection title="Interests">
                <NavLink title="Movies".to_string() path="/movies".to_string()/>
                <NavLink title="Music".to_string()/>
                <NavLink title="Food".to_string()/>
            </NavSection>
            </ul>
            </nav>
        </div>
        </div>
            }}
        }
        </Suspense>
    }
}

#[component]
fn NavLink(title: String, #[prop(optional)] path: String) -> impl IntoView {
    view! {
        <a href={path} class="text-secondary ">
            <li>
                {title}
            </li>
        </a>
    }
}

#[component]
fn NavSection(title: &'static str, children: Children) -> impl IntoView {
    view! {
        <li>
            <h2 class="text-lg text-text-600">{title}</h2>
            <ul class="flex flex-col gap-1">
                {children()}
            </ul>
        </li>
    }
}
