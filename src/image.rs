const BASE: &str = "https://image.tmdb.org/t/p";

macro_rules! paths {
    ($($name:ident { $($method:ident = $key:literal),* $(,)? }),* $(,)?) => {$(
        /// an image path; call a size method for the full url
        #[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            $(
                pub fn $method(&self) -> String {
                    format!("{BASE}/{}{}", $key, self.0)
                }
            )*

            /// the full url at the original upload size
            pub fn original(&self) -> String {
                format!("{BASE}/original{}", self.0)
            }

            /// the raw `*_path` value, for storage
            pub fn path(&self) -> &str {
                &self.0
            }
        }
    )*};
}

paths! {
    Poster { w92 = "w92", w154 = "w154", w185 = "w185", w342 = "w342", w500 = "w500", w780 = "w780" },
    Backdrop { w300 = "w300", w780 = "w780", w1280 = "w1280" },
    Profile { w45 = "w45", w185 = "w185", h632 = "h632" },
    Still { w92 = "w92", w185 = "w185", w300 = "w300" },
    Logo { w45 = "w45", w92 = "w92", w154 = "w154", w185 = "w185", w300 = "w300", w500 = "w500" },
}
