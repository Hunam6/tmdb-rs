use serde::{Deserialize, Deserializer};
use time::macros::format_description;
use time::Date;

/// TMDB writes unknown dates as "" rather than null
pub(crate) fn opt_date<'de, D: Deserializer<'de>>(d: D) -> Result<Option<Date>, D::Error> {
    match Option::<String>::deserialize(d)?.filter(|s| !s.is_empty()) {
        Some(s) => Date::parse(&s, format_description!("[year]-[month]-[day]"))
            .map(Some)
            .map_err(serde::de::Error::custom),
        None => Ok(None),
    }
}
