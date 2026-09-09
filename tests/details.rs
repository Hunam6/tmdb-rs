use tmdb_rs::{Client, Error};
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn mock_client(server: &MockServer) -> Client {
    Client::with_api_key("key").with_base_url(server.uri())
}

#[tokio::test]
async fn appends_fill_typed_slots() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/movie/550"))
        .and(query_param("append_to_response", "credits,similar"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": 550,
            "title": "Fight Club",
            "original_title": "Fight Club",
            "overview": "A ticking-Loss insomniac...",
            "tagline": "Mischief. Mayhem. Soap.",
            "release_date": "1999-10-15",
            "vote_average": 8.4,
            "vote_count": 28000,
            "popularity": 61.0,
            "runtime": 139,
            "original_language": "en",
            "poster_path": "/pB8BM7pdSp6B6Ih7QZ4DrQ3PmJK.jpg",
            "backdrop_path": "/fCayJrkfRaCRCTh8GqN30f8oyQF.jpg",
            "genres": [{"id": 18, "name": "Drama"}],
            "imdb_id": "tt0137523",
            "homepage": "",
            "status": "Released",
            "budget": 63000000,
            "revenue": 100853753,
            "adult": false,
            "credits": {
                "id": 550,
                "cast": [{
                    "id": 819,
                    "name": "Edward Norton",
                    "character": "The Narrator",
                    "profile_path": null,
                    "order": 0,
                    "credit_id": "52fe4250c3a36847f80149f3"
                }],
                "crew": []
            },
            "similar": {
                "page": 1,
                "results": [],
                "total_pages": 0,
                "total_results": 0
            }
        })))
        .mount(&server)
        .await;

    let movie = mock_client(&server)
        .movie(550)
        .with_credits()
        .with_similar()
        .send()
        .await
        .unwrap();

    assert_eq!(movie.title, "Fight Club");
    assert_eq!(movie.release_date.unwrap().year(), 1999);
    assert_eq!(movie.credits.cast[0].name, "Edward Norton");
    assert_eq!(movie.similar.page, 1);
    // not requested, and the type says so
    let _: () = movie.images;
    let _: () = movie.keywords;
}

#[tokio::test]
async fn unrequested_appends_stay_unit() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/movie/550"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": 550,
            "title": "Fight Club",
            "original_title": "Fight Club",
            "overview": "",
            "tagline": "",
            "release_date": "",
            "vote_average": 8.4,
            "vote_count": 28000,
            "popularity": 61.0,
            "runtime": 139,
            "original_language": "en",
            "poster_path": null,
            "backdrop_path": null,
            "genres": [],
            "imdb_id": null,
            "homepage": "",
            "status": "Released",
            "budget": 0,
            "revenue": 0,
            "adult": false
        })))
        .mount(&server)
        .await;

    let movie = mock_client(&server).movie(550).send().await.unwrap();
    let _: () = movie.credits;
    // TMDB's empty-string date
    assert_eq!(movie.release_date, None);
}

#[tokio::test]
async fn requested_but_missing_append_is_an_error() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/movie/550"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": 550,
            "title": "Fight Club",
            "original_title": "Fight Club",
            "overview": "",
            "tagline": "",
            "release_date": null,
            "vote_average": 8.4,
            "vote_count": 28000,
            "popularity": 61.0,
            "runtime": 139,
            "original_language": "en",
            "poster_path": null,
            "backdrop_path": null,
            "genres": [],
            "imdb_id": null,
            "homepage": "",
            "status": "Released",
            "budget": 0,
            "revenue": 0,
            "adult": false
        })))
        .mount(&server)
        .await;

    let error = mock_client(&server)
        .movie(550)
        .with_credits()
        .send()
        .await
        .unwrap_err();
    assert!(matches!(error, Error::MissingAppend("credits")));
}

#[tokio::test]
async fn tmdb_404_maps_to_not_found() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/movie/0"))
        .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
            "success": false,
            "status_code": 34,
            "status_message": "The resource you requested could not be found."
        })))
        .mount(&server)
        .await;

    let error = mock_client(&server).movie(0).send().await.unwrap_err();
    assert!(matches!(error, Error::NotFound));
}

#[tokio::test]
async fn search_discriminates_by_media_type() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/search/multi"))
        .and(query_param("query", "fight"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "page": 1,
            "total_pages": 1,
            "total_results": 2,
            "results": [
                {
                    "media_type": "movie",
                    "id": 550,
                    "title": "Fight Club",
                    "original_title": "Fight Club",
                    "overview": "",
                    "release_date": "1999-10-15",
                    "poster_path": null,
                    "backdrop_path": null,
                    "vote_average": 8.4,
                    "vote_count": 1,
                    "popularity": 1.0,
                    "genre_ids": [18],
                    "original_language": "en",
                    "adult": false,
                    "video": false
                },
                {
                    "media_type": "tv",
                    "id": 106379,
                    "name": "Fallout",
                    "original_name": "Fallout",
                    "overview": "",
                    "first_air_date": "2024-04-10",
                    "poster_path": null,
                    "backdrop_path": null,
                    "vote_average": 8.4,
                    "vote_count": 1,
                    "popularity": 1.0,
                    "genre_ids": [],
                    "original_language": "en",
                    "origin_country": ["US"],
                    "adult": false
                }
            ]
        })))
        .mount(&server)
        .await;

    let page = mock_client(&server)
        .search("fight")
        .send()
        .await
        .unwrap();
    assert_eq!(page.results.len(), 2);
    assert!(matches!(page.results[0], tmdb_rs::MultiResult::Movie(_)));
    assert!(matches!(page.results[1], tmdb_rs::MultiResult::Tv(_)));
}
