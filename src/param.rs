use std::fmt;

use crate::{Country, Date, GuestSessionId, Language, SessionId};

/// a query param value; slices join with commas, as TMDB expects
pub trait ToParam {
    fn to_param(&self) -> String;
}

macro_rules! to_param {
    ($($t:ty),* $(,)?) => {$(
        impl ToParam for $t {
            fn to_param(&self) -> String {
                self.to_string()
            }
        }
    )*};
}

to_param!(
    u32,
    u64,
    i32,
    i64,
    f64,
    bool,
    &str,
    String,
    Language,
    Country,
    Date,
    SessionId,
    GuestSessionId
);

impl<T: fmt::Display> ToParam for &[T] {
    fn to_param(&self) -> String {
        self.iter()
            .map(|item| item.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }
}

impl<T: fmt::Display> ToParam for Vec<T> {
    fn to_param(&self) -> String {
        self.as_slice().to_param()
    }
}
