use leptos::*;
use serde::*;
use web_sys::Event;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Movie {
    pub title: String,
    pub description: String,
    pub rating: u8,
}

#[island]
pub fn MovieList(movies: Vec<Movie>) -> impl IntoView {
    let (min_rating, set_min_rating) = create_signal::<u8>(0);
    let on_change = move |event: Event| set_min_rating(event_target_value(&event).parse().unwrap());
    view! {
        <div class="flex flex-col gap-4">
            <div class="self-end flex flex-row align-center gap-1">
                <input id="min_rating" type="range" min="0" max="10" step="1" value=min_rating on:input=on_change />
                <label for="min_rating">"≥"{min_rating}</label>
            </div>
            <div class="flex flex-row flex-wrap px-8">
                {movies.iter().cloned().map(|movie| {
                    view! {
                        <div class="movie w-[20%]" class:hidden={move|| {min_rating() > movie.rating}} inner_html=movie.description.clone() />
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}

#[island]
pub fn Counter() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}
