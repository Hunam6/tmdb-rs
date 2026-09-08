/// a plain list/search endpoint: `Client` method, builder with one method per
/// optional query param, `send()`. required query params become method args,
/// path params fill the path's `{}` in order
macro_rules! endpoint {
    // no sections
    (
        $(#[$meta:meta])*
        $method:ident ($($pn:ident : $pt:ty),* $(,)?): GET $path:literal => $resp:ty
    ) => {
        endpoint! {
            $(#[$meta])*
            $method($($pn : $pt),*): GET $path => $resp { required {} params {} }
        }
    };
    // params section only
    (
        $(#[$meta:meta])*
        $method:ident ($($pn:ident : $pt:ty),* $(,)?): GET $path:literal => $resp:ty {
            params { $($qp:ident : $qt:ty),* $(,)? }
        }
    ) => {
        endpoint! {
            $(#[$meta])*
            $method($($pn : $pt),*): GET $path => $resp {
                required {}
                params { $($qp : $qt),* }
            }
        }
    };
    // full form
    (
        $(#[$meta:meta])*
        $method:ident ($($pn:ident : $pt:ty),* $(,)?): GET $path:literal => $resp:ty {
            required { $($rq:ident : $rt:ty),* $(,)? }
            params { $($qp:ident : $qt:ty),* $(,)? }
        }
    ) => {
        ::paste::paste! {
            #[doc = concat!("builder for `GET ", $path, "`")]
            pub struct [<$method:camel Request>] {
                client: $crate::Client,
                $($pn: $pt,)*
                pairs: ::std::vec::Vec<(&'static str, ::std::string::String)>,
            }

            impl $crate::Client {
                $(#[$meta])*
                pub fn $method(&self, $($pn: $pt,)* $($rq: $rt),*) -> [<$method:camel Request>] {
                    let pairs = ::std::vec![$((stringify!($rq), $rq.to_string())),*];
                    [<$method:camel Request>] { client: self.clone(), $($pn,)* pairs }
                }
            }

            impl [<$method:camel Request>] {
                $(
                    pub fn $qp(mut self, value: $qt) -> Self {
                        self.pairs.push((stringify!($qp), value.to_string()));
                        self
                    }
                )*

                pub async fn send(self) -> $crate::Result<$resp> {
                    let path = ::std::format!($path $(, self.$pn)*);
                    self.client.get(&path, &self.pairs).await
                }
            }
        }
    };
}

/// a detail endpoint with compile-time append_to_response: each `with_*`
/// fills one slot of the response type, and a requested-but-missing payload
/// is an error rather than a silent None. field attributes (serde) apply to
/// the private shadow struct that deserialization goes through; on the public
/// struct they're stripped via cfg_attr, since it has no Deserialize derive
macro_rules! details {
    // no params section
    (
        $(#[$meta:meta])*
        $method:ident ($($pn:ident : $pt:ty),* $(,)?): GET $path:literal => $resp:ident {
            base { $($(#[$bm:meta])* pub $bf:ident : $bt:ty),* $(,)? }
            appends { $($an:ident : $at:ty),* $(,)? }
        }
    ) => {
        details! {
            $(#[$meta])*
            $method($($pn : $pt),*): GET $path => $resp {
                params {}
                base { $($(#[$bm])* pub $bf : $bt),* }
                appends { $($an : $at),* }
            }
        }
    };
    // full form
    (
        $(#[$meta:meta])*
        $method:ident ($($pn:ident : $pt:ty),* $(,)?): GET $path:literal => $resp:ident {
            params { $($qp:ident : $qt:ty),* $(,)? }
            base { $($(#[$bm:meta])* pub $bf:ident : $bt:ty),* $(,)? }
            appends { $($an:ident : $at:ty),* $(,)? }
        }
    ) => {
        ::paste::paste! {
            $(#[$meta])*
            #[derive(Debug, Clone)]
            pub struct $resp<$([<A $an:camel>] = ()),*> {
                $(
                    $(#[cfg_attr(any(), $bm)])*
                    pub $bf: $bt,
                )*
                $(
                    /// appended payload — `()` unless requested via the builder
                    pub $an: [<A $an:camel>],
                )*
            }

            #[doc = concat!("builder for `GET ", $path, "`; each `with_*` fills one append slot")]
            pub struct [<$resp Request>]<$([<A $an:camel>] = ()),*> {
                client: $crate::Client,
                $($pn: $pt,)*
                pairs: ::std::vec::Vec<(&'static str, ::std::string::String)>,
                _appends: ::std::marker::PhantomData<fn() -> ($([<A $an:camel>]),*)>,
            }

            impl $crate::Client {
                $(#[$meta])*
                pub fn $method(&self, $($pn: $pt),*) -> [<$resp Request>] {
                    [<$resp Request>] {
                        client: self.clone(),
                        $($pn,)*
                        pairs: ::std::vec::Vec::new(),
                        _appends: ::std::marker::PhantomData,
                    }
                }
            }

            impl<$([<A $an:camel>]),*> [<$resp Request>]<$([<A $an:camel>]),*> {
                $(
                    pub fn $qp(mut self, value: $qt) -> Self {
                        self.pairs.push((stringify!($qp), value.to_string()));
                        self
                    }
                )*
            }

            details!(@withs [<$resp Request>] [$($pn,)*] [] [$($an : $at,)*]);

            impl<$([<A $an:camel>]: $crate::Append),*> [<$resp Request>]<$([<A $an:camel>]),*> {
                pub async fn send(self) -> $crate::Result<$resp<$([<A $an:camel>]),*>> {
                    let mut pairs = self.pairs;
                    let mut append = ::std::string::String::new();
                    $(
                        if ![<A $an:camel>]::ABSENT {
                            append.push_str(stringify!($an));
                            append.push(',');
                        }
                    )*
                    if !append.is_empty() {
                        append.pop();
                        pairs.push(("append_to_response", append));
                    }
                    let path = ::std::format!($path $(, self.$pn)*);
                    $resp::<$([<A $an:camel>]),*>::fetch(&self.client, &path, &pairs).await
                }
            }

            const _: () = {
                #[derive(::serde::Deserialize)]
                struct Shadow {
                    $($(#[$bm])* $bf: $bt,)*
                    $($an: ::std::option::Option<::serde_json::Value>,)*
                }

                impl<$([<A $an:camel>]: $crate::Append),*> $resp<$([<A $an:camel>]),*> {
                    pub(crate) async fn fetch(
                        client: &$crate::Client,
                        path: &str,
                        pairs: &[(&'static str, ::std::string::String)],
                    ) -> $crate::Result<Self> {
                        let shadow: Shadow = client.get(path, pairs).await?;
                        Ok(Self {
                            $($bf: shadow.$bf,)*
                            $($an: [<A $an:camel>]::from_json(stringify!($an), shadow.$an)?,)*
                        })
                    }
                }

                // the default (nothing appended) state is directly deserializable,
                // so plain endpoints can reuse the type and users can cache it
                impl<'de> ::serde::Deserialize<'de> for $resp {
                    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> ::std::result::Result<Self, D::Error> {
                        let shadow = Shadow::deserialize(d)?;
                        Ok(Self {
                            $($bf: shadow.$bf,)*
                            $($an: <() as $crate::Append>::from_json(stringify!($an), shadow.$an)
                                .map_err(::serde::de::Error::custom)?,)*
                        })
                    }
                }
            };
        }
    };

    // one with_* per append slot, available only while that slot is ()
    (@withs $req:ident [$($pn:ident,)*] [$($bn:ident : $bt:ty,)*] []) => {};
    (@withs $req:ident [$($pn:ident,)*] [$($bn:ident : $bt:ty,)*] [$an:ident : $at:ty, $($rest:ident : $rt:ty,)*]) => {
        ::paste::paste! {
            impl<$([<A $bn:camel>],)* $([<A $rest:camel>],)*> $req<$([<A $bn:camel>],)* (), $([<A $rest:camel>],)*> {
                #[doc = concat!("append `", stringify!($an), "` to the response")]
                pub fn [<with_ $an>](self) -> $req<$([<A $bn:camel>],)* $at, $([<A $rest:camel>],)*> {
                    $req {
                        client: self.client,
                        $($pn: self.$pn,)*
                        pairs: self.pairs,
                        _appends: ::std::marker::PhantomData,
                    }
                }
            }
        }
        details!(@withs $req [$($pn,)*] [$($bn : $bt,)* $an : $at,] [$($rest : $rt,)*]);
    };
}
