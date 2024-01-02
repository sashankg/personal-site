use leptos::*;
use std::{cell::RefCell, collections::HashSet, rc::Rc};

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

    if let Some(context) = use_context::<Rc<RefCell<RouteSet>>>() {
        (*context).borrow_mut().routes.insert(path.to_string());
    }

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

#[derive(Clone, Default)]
struct RouteSet {
    routes: HashSet<String>,
}

pub fn generate_route_list<IV>(app_fn: impl FnOnce() -> IV + 'static + Clone) -> HashSet<String>
where
    IV: IntoView + 'static,
{
    create_runtime();
    let route_list = Rc::new(RefCell::new(RouteSet::default()));
    provide_context(route_list.clone());

    let _ = app_fn().into_view();

    return (*route_list).borrow().routes.clone();
}
