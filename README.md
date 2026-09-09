# tmdb-rs

An async, fully-typed client for [TMDB](https://www.themoviedb.org) (the movie
database), covering API v3 and v4.

The signature feature is compile-time `append_to_response`: each `with_*`
call fills one slot of the response type, so an appended payload is a plain
field rather than an `Option`:

```rust,no_run
let tmdb = tmdb_rs::Client::new(std::env::var("TMDB_READ_TOKEN").unwrap());

let movie = tmdb.movie(550).with_credits().with_similar().send().await?;
println!("{}", movie.name);
println!("{} cast members", movie.credits.cast.len());
println!("{} similar movies", movie.similar.results.len());
// movie.images is () — it wasn't requested, and the type says so
```

## Highlights

- **Everything typed, everything covered** — all v3 namespaces (movies, tv,
  seasons/episodes, people, search, discover, account, lists, ...) and the
  v4 auth flow, list CRUD and account endpoints.
- **Compile-time appends** — request `credits`, `images`, `watch/providers`,
  ... and read them off the response as non-optional fields.
- **Two credentials, no mix-ups** — v3 `SessionId` and v4 `AccessToken` are
  distinct types; the v4 surface lives behind `client.v4(&token)`.
- **Pagination as a stream** — with the `stream` feature, any paginated
  endpoint drains via `.into_stream()`.
- **Sane defaults** — rustls-only TLS, automatic 429 retry honoring
  `Retry-After`, typed `time::Date` dates, image URL builders.

## Usage

```rust,no_run
use tmdb_rs::{Client, Language};

# async fn example() -> tmdb_rs::Result<()> {
// v4 read access token (or Client::with_api_key for a v3 key)
let tmdb = Client::new("eyJ...");

// search
let page = tmdb.search_movies("fight club").send().await?;
for movie in &page.results {
    println!("{} ({:?})", movie.name, movie.release_date);
}

// details with appends
let tv = tmdb
    .tv(1399)
    .language(Language::Fra)
    .with_credits()
    .with_content_ratings()
    .send()
    .await?;
println!("{}: {} seasons", tv.name, tv.number_of_seasons.unwrap_or(0));

// v4: the auth flow and a user's lists
let v4 = tmdb.v4(&access_token);
let lists = v4.v4_account_lists("account-id").send().await?;
# Ok(())
# }
```

## Streaming pages

```rust,ignore
use futures_util::StreamExt;

let mut stream = tmdb.movie_popular().into_stream();
while let Some(movie) = stream.next().await {
    println!("{}", movie?.title);
}
```

## Features

| feature | what it adds |
| ------- | ------------ |
| `stream` | `into_stream()` on paginated GET builders (`futures-core` only) |

## License

MIT
