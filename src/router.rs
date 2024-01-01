use leptos::*;

struct RouterContext {
    route: String,
    base_url: String,
}

#[component]
pub fn Router(route: String, base_url: String, children: Children) -> impl IntoView {
    provide_context(store_value(RouterContext { route, base_url }));
    children()
}

#[component]
pub fn Route(path: &'static str, children: ChildrenFn) -> impl IntoView {
    let context = use_context::<StoredValue<RouterContext>>();
    let matches_path = context.map_or(false, |cx| cx.with_value(|router| router.route == path));
    view! {
        <Show when={move || matches_path}>
            {children()}
        </Show>
    }
}

#[component]
pub fn Link(path: String, class: &'static str, children: Children) -> impl IntoView {
    let context = use_context::<StoredValue<RouterContext>>();
    let href = context.map_or_else(
        || path.clone(),
        |cx| cx.with_value(|router| format!("{}{}", router.base_url, path)),
    );
    view! {
        <a href=href class=class>
            {children()}
        </a>
    }
}
