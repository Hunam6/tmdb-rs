use crate::{CountryCode, Date, GuestSessionId, Language, SessionId};

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

to_param!(u32, u64, i32, i64, f64, bool, &str, String, Date, SessionId, GuestSessionId);

impl ToParam for Language {
    fn to_param(&self) -> String {
        // TMDB wants 639-1; a few languages only have a 639-3 code
        self.to_639_1().unwrap_or_else(|| self.to_639_3()).into()
    }
}

impl ToParam for CountryCode {
    fn to_param(&self) -> String {
        self.alpha2().into()
    }
}

/// enums whose wire form a serde derive already spells out
pub(crate) fn serde_param(value: &impl serde::Serialize) -> String {
    match serde_json::to_value(value).expect("a unit variant") {
        serde_json::Value::String(s) => s,
        _ => unreachable!("unit variants serialize to strings"),
    }
}

impl<T: ToParam> ToParam for &[T] {
    fn to_param(&self) -> String {
        self.iter()
            .map(ToParam::to_param)
            .collect::<Vec<_>>()
            .join(",")
    }
}

impl<T: ToParam> ToParam for Vec<T> {
    fn to_param(&self) -> String {
        self.as_slice().to_param()
    }
}
