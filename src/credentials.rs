use std::fmt;

macro_rules! credential {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Self {
                Self(value.into())
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(&self.0)
            }
        }
    };
}

credential! {
    /// a v3 session id, from the authentication flow
    SessionId
}

#[cfg(feature = "v4")]
credential! {
    /// a v4 user access token, from the v4 auth flow
    AccessToken
}

credential! {
    /// a v3 guest session id
    GuestSessionId
}
