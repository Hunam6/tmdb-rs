#![cfg(feature = "stream")]

use futures_util::StreamExt;
use tmdb_rs::Client;
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn movie(id: u64, title: &str) -> serde_json::Value {
    serde_json::json!({
        "id": id,
        "title": title,
        "original_title": title,
        "overview": "",
        "poster_path": null,
        "backdrop_path": null,
        "release_date": "",
        "vote_average": 0.0,
        "vote_count": 0,
        "popularity": 0.0,
        "original_language": "en",
        "genre_ids": [],
        "adult": false,
        "video": false,
    })
}

#[tokio::test]
async fn stream_walks_every_page() {
    let server = MockServer::start().await;
    for (page, titles) in [(1, &["One", "Two"][..]), (2, &["Three"][..])] {
        Mock::given(method("GET"))
            .and(path("/movie/popular"))
            .and(query_param("page", page.to_string()))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "page": page,
                "results": titles.iter().enumerate().map(|(i, t)| movie(i as u64, t)).collect::<Vec<_>>(),
                "total_pages": 2,
                "total_results": 3,
            })))
            .mount(&server)
            .await;
    }

    let client = Client::with_api_key("key").with_base_url(server.uri());
    let titles: Vec<String> = client
        .movie_popular()
        .into_stream()
        .map(|movie| movie.unwrap().title)
        .collect()
        .await;
    assert_eq!(titles, ["One", "Two", "Three"]);
}
