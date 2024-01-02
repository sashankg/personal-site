use leptos::*;

#[component]
pub fn Page(title: &'static str, children: Children) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-1 justify-center">
            <div class="h-[20vh] flex flex-row items-end">
                <h2 class="text-primary text-2xl font-medium font-display">{title}</h2>
            </div>
            {children()}
        </div>

    }
}
