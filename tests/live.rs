//! against the real API; run with `TMDB_API_KEY=... cargo test --test live`
//! errors are scrubbed because reqwest's Display includes the url, key and all

#![allow(clippy::result_large_err)]

use tmdb_rs::Client;

fn client() -> Option<Client> {
    let key = std::env::var("TMDB_API_KEY").ok()?;
    Some(Client::with_api_key(key))
}

macro_rules! live {
    ($body:expr) => {{
        let Some(client) = client() else {
            eprintln!("TMDB_API_KEY unset, skipping");
            return;
        };
        $body(client)
            .await
            .map_err(|error| {
                // reqwest errors carry the url, api key included
                let key = std::env::var("TMDB_API_KEY").unwrap();
                format!("{error:?}").replace(&key, "***")
            })
            .unwrap()
    }};
}

#[tokio::test]
async fn movie_with_appends() {
    let movie = live!(|tmdb: Client| async move {
        tmdb.movie(550)
            .with_credits()
            .with_release_dates()
            .with_similar()
            .send()
            .await
    });
    assert_eq!(movie.name, "Fight Club");
    assert!(!movie.credits.cast.is_empty());
    assert!(!movie.release_dates.results.is_empty());
    assert!(!movie.similar.results.is_empty());
}

#[tokio::test]
async fn tv_with_appends() {
    let show = live!(|tmdb: Client| async move {
        tmdb.tv(1399)
            .with_content_ratings()
            .with_credits()
            .with_similar()
            .send()
            .await
    });
    assert_eq!(show.name, "Game of Thrones");
    assert!(!show.seasons.is_empty());
    assert!(!show.content_ratings.results.is_empty());
}

#[tokio::test]
async fn season_and_search() {
    live!(|tmdb: Client| async move {
        let season = tmdb.tv_season(1399, 1).send().await?;
        assert_eq!(season.episodes.len(), 10);

        let page = tmdb.search("fight club").send().await?;
        assert!(!page.results.is_empty());

        let genres = tmdb.movie_genres().send().await?;
        assert!(!genres.genres.is_empty());
        Ok::<(), tmdb_rs::Error>(())
    });
}
