use ::macros::async_component;
use leptos::*;

use crate::components::Page;
use crate::islands::{Movie, MovieList};

#[async_component]
pub async fn MoviesPage() -> impl IntoView {
    let content = reqwest::get("https://letterboxd.com/sashankg/rss/")
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap();
    let channel = rss::Channel::read_from(&content[..]).unwrap();
    let movies = channel
        .items()
        .iter()
        .flat_map(|item| {
            let ext = item.extensions().get("letterboxd")?;
            Some(Movie {
                title: ext.get("filmTitle")?.first()?.value()?.to_string(),
                description: item.description()?.to_string(),
                rating: ext
                    .get("memberRating")?
                    .first()?
                    .value()?
                    .parse::<f32>()
                    .ok()
                    .map(|score| (score * 2.0).round() as u8)?,
            })
        })
        .collect::<Vec<Movie>>();

    return view! {
        <Page title="Movies">
            <MovieList movies={movies} />
        </Page>
    };
}
