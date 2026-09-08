use std::borrow::Cow;
use std::fmt;

macro_rules! code {
    ($(#[$meta:meta])* $name:ident { $($const:ident = $value:literal),* $(,)? }) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct $name(Cow<'static, str>);

        impl $name {
            $(pub const $const: Self = Self(Cow::Borrowed($value));)*

            /// any code the consts don't cover
            pub fn new(code: impl Into<String>) -> Self {
                Self(Cow::Owned(code.into()))
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(&self.0)
            }
        }
    };
}

code! {
    /// ISO 639-1 language code, e.g. for `language` params
    Language {
        ENGLISH = "en",
        FRENCH = "fr",
        GERMAN = "de",
        SPANISH = "es",
        ITALIAN = "it",
        JAPANESE = "ja",
        KOREAN = "ko",
        PORTUGUESE = "pt",
        RUSSIAN = "ru",
        CHINESE = "zh",
    }
}

code! {
    /// ISO 3166-1 country code, e.g. for `region`/`watch_region` params
    Country {
        US = "US",
        GB = "GB",
        FRANCE = "FR",
        GERMANY = "DE",
        SPAIN = "ES",
        ITALY = "IT",
        JAPAN = "JP",
        KOREA = "KR",
        BRAZIL = "BR",
        CANADA = "CA",
    }
}
