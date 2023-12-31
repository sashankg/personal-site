use leptos::*;

use crate::components::page::Page;

#[component]
pub fn ProjectsPage() -> impl IntoView {
    view! {
        <Page title="Projects">
            {"list of projects"}
        </Page>
    }
}
