# tmdb-rs

An async Rust-idiomatic client for [TMDB](https://www.themoviedb.org), covering every
endpoint of API v3 and v4.

`append_to_response` is checked at compile time: each `with_*` call fills a
slot in the response *type*, so what you asked for is a plain field, and what
you didn't ask for doesn't exist:

```rust,no_run
let tmdb = tmdb_rs::Client::with_read_token(std::env::var("TMDB_READ_TOKEN").unwrap());

let movie = tmdb.movie(920).with_credits().with_similar().send().await?;
println!("{} cast members", movie.credits.cast.len()); // not an Option
// movie.images is () — it wasn't requested, and the type says so
```

## Why this crate

- **Complete** — all of v3 plus the v4 auth flow, list CRUD and account
  endpoints; deprecated TMDB surface is marked
- **Idiomatic** — `time::Date` dates, `isolang`/`isocountry` locales, enums
  over magic strings, tagged unions (like multi-search results) as Rust
  enums instead of structs full of `Option`s
- **Well-behaved** — rustls-only TLS, automatic 429 retry honoring
  `Retry-After`, image URL builders (`.w500()`, `.original()`)
- **Streamable** — paginated endpoints drain item by item via
  `.into_stream()` (feature `stream`)

## Features

| feature | what it adds |
| ------- | ------------ |
| `stream` | `into_stream()` on paginated GET builders |
| `v4` | the v4 API: auth flow, list CRUD, account lists |

## License

MIT
