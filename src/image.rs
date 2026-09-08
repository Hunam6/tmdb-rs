const BASE: &str = "https://image.tmdb.org/t/p";

macro_rules! sizes {
    ($($name:ident { $($variant:ident = $key:literal),* $(,)? }),* $(,)?) => {$(
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum $name {
            $($variant,)*
            Original,
        }

        impl $name {
            /// the full image url for a `*_path` value
            pub fn url(self, path: &str) -> String {
                    let size = match self {
                    $(Self::$variant => $key,)*
                    Self::Original => "original",
                };
                format!("{BASE}/{size}{path}")
            }
        }
    )*};
}

sizes! {
    Poster { W92 = "w92", W154 = "w154", W185 = "w185", W342 = "w342", W500 = "w500", W780 = "w780" },
    Backdrop { W300 = "w300", W780 = "w780", W1280 = "w1280" },
    Profile { W45 = "w45", W185 = "w185", H632 = "h632" },
    Still { W92 = "w92", W185 = "w185", W300 = "w300" },
    Logo { W45 = "w45", W92 = "w92", W154 = "w154", W185 = "w185", W300 = "w300", W500 = "w500" },
}
