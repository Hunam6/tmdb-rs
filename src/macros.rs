/// a plain endpoint: `Client` method, builder with one method per optional
/// query param, `send()`. path params fill the path's `{}` in order, required
/// query params and body fields become method args
macro_rules! endpoint {
    // bare
    (
        $(@gen $recv:ident,)?
        $(#[$meta:meta])*
        $method:ident ($($pn:ident : $pt:ty),* $(,)?): $verb:ident $path:literal => $resp:ty
    ) => {
        endpoint! {
            $(@gen $recv,)?
            $(#[$meta])*
            $method($($pn : $pt),*): $verb $path => $resp { required {} params {} body {} }
        }
    };
    // params only
    (
        $(@gen $recv:ident,)?
        $(#[$meta:meta])*
        $method:ident ($($pn:ident : $pt:ty),* $(,)?): $verb:ident $path:literal => $resp:ty {
            params { $($qp:ident : $qt:ty),* $(,)? }
        }
    ) => {
        endpoint! {
            $(@gen $recv,)?
            $(#[$meta])*
            $method($($pn : $pt),*): $verb $path => $resp {
                required {}
                params { $($qp : $qt),* }
                body {}
            }
        }
    };
    // required + params
    (
        $(@gen $recv:ident,)?
        $(#[$meta:meta])*
        $method:ident ($($pn:ident : $pt:ty),* $(,)?): $verb:ident $path:literal => $resp:ty {
            required { $($rq:ident : $rt:ty),* $(,)? }
            params { $($qp:ident : $qt:ty),* $(,)? }
        }
    ) => {
        endpoint! {
            $(@gen $recv,)?
            $(#[$meta])*
            $method($($pn : $pt),*): $verb $path => $resp {
                required { $($rq : $rt),* }
                params { $($qp : $qt),* }
                body {}
            }
        }
    };
    // body only
    (
        $(@gen $recv:ident,)?
        $(#[$meta:meta])*
        $method:ident ($($pn:ident : $pt:ty),* $(,)?): $verb:ident $path:literal => $resp:ty {
            body { $($bn:ident : $bt:ty),* $(,)? }
        }
    ) => {
        endpoint! {
            $(@gen $recv,)?
            $(#[$meta])*
            $method($($pn : $pt),*): $verb $path => $resp {
                required {}
                params {}
                body { $($bn : $bt),* }
            }
        }
    };
    // required only
    (
        $(@gen $recv:ident,)?
        $(#[$meta:meta])*
        $method:ident ($($pn:ident : $pt:ty),* $(,)?): $verb:ident $path:literal => $resp:ty {
            required { $($rq:ident : $rt:ty),* $(,)? }
        }
    ) => {
        endpoint! {
            $(@gen $recv,)?
            $(#[$meta])*
            $method($($pn : $pt),*): $verb $path => $resp {
                required { $($rq : $rt),* }
                params {}
                body {}
            }
        }
    };
    // params + body
    (
        $(@gen $recv:ident,)?
        $(#[$meta:meta])*
        $method:ident ($($pn:ident : $pt:ty),* $(,)?): $verb:ident $path:literal => $resp:ty {
            params { $($qp:ident : $qt:ty),* $(,)? }
            body { $($bn:ident : $bt:ty),* $(,)? }
        }
    ) => {
        endpoint! {
            $(@gen $recv,)?
            $(#[$meta])*
            $method($($pn : $pt),*): $verb $path => $resp {
                required {}
                params { $($qp : $qt),* }
                body { $($bn : $bt),* }
            }
        }
    };
    // required + body
    (
        $(@gen $recv:ident,)?
        $(#[$meta:meta])*
        $method:ident ($($pn:ident : $pt:ty),* $(,)?): $verb:ident $path:literal => $resp:ty {
            required { $($rq:ident : $rt:ty),* $(,)? }
            body { $($bn:ident : $bt:ty),* $(,)? }
        }
    ) => {
        endpoint! {
            $(@gen $recv,)?
            $(#[$meta])*
            $method($($pn : $pt),*): $verb $path => $resp {
                required { $($rq : $rt),* }
                params {}
                body { $($bn : $bt),* }
            }
        }
    };
    // full form, on the v3 client
    (
        $(#[$meta:meta])*
        $method:ident ($($pn:ident : $pt:ty),* $(,)?): $verb:ident $path:literal => $resp:ty {
            required { $($rq:ident : $rt:ty),* $(,)? }
            params { $($qp:ident : $qt:ty),* $(,)? }
            body { $($bn:ident : $bt:ty),* $(,)? }
        }
    ) => {
        endpoint!(@gen Client, $(#[$meta])* $method($($pn : $pt),*): $verb $path => $resp {
            required { $($rq : $rt),* }
            params { $($qp : $qt),* }
            body { $($bn : $bt),* }
        });
    };
    // generator, with the receiver made explicit so v4 can share it
    (
        @gen $recv:ident, $(#[$meta:meta])*
        $method:ident ($($pn:ident : $pt:ty),* $(,)?): $verb:ident $path:literal => $resp:ty {
            required { $($rq:ident : $rt:ty),* $(,)? }
            params { $($qp:ident : $qt:ty),* $(,)? }
            body { $($bn:ident : $bt:ty),* $(,)? }
        }
    ) => {
        ::paste::paste! {
            #[doc = concat!("builder for `", stringify!($verb), " ", $path, "`")]
            #[derive(Clone)]
            pub struct [<$method:camel Request>] {
                client: $crate::$recv,
                path: ::std::string::String,
                pairs: ::std::vec::Vec<(&'static str, ::std::string::String)>,
                _body: ::serde_json::Value,
            }

            impl $crate::$recv {
                $(#[$meta])*
                pub fn $method(&self, $($pn: $pt,)* $($rq: $rt,)* $($bn: $bt),*) -> [<$method:camel Request>] {
                    let path = ::std::format!($path $(, $pn)*);
                    let pairs = ::std::vec![$((stringify!($rq), $crate::ToParam::to_param(&$rq))),*];
                    let _body = ::serde_json::json!({ $(stringify!($bn): $bn),* });
                    [<$method:camel Request>] { client: self.clone(), path, pairs, _body }
                }
            }

            impl [<$method:camel Request>] {
                $(
                    pub fn $qp(mut self, value: $qt) -> Self {
                        self.pairs.push((stringify!($qp), $crate::ToParam::to_param(&value)));
                        self
                    }
                )*

                pub async fn send(self) -> $crate::Result<$resp> {
                    endpoint!(@send $verb self)
                }
            }
        }
        endpoint!(@stream $verb $method $resp);
    };

    (@send GET $self:ident) => {
        $self.client.get(&$self.path, &$self.pairs).await
    };
    (@send $verb:ident $self:ident) => {
        ::paste::paste! { $self.client.[<$verb:lower>](&$self.path, &$self.pairs, &$self._body).await }
    };

    // GET builders gain into_stream; it only implements Stream when the
    // response is a Page, which the macro can't see through :ty fragments
    (@stream GET $method:ident $resp:ty) => {
        ::paste::paste! {
            impl [<$method:camel Request>] {
                /// every page of the endpoint as a stream of items
                #[cfg(feature = "stream")]
                pub fn into_stream(self) -> $crate::stream::PageStream<$resp> {
                    $crate::stream::page_stream(move |page: u32| {
                        let mut request = self.clone();
                        request.pairs.retain(|(key, _)| *key != "page");
                        request.pairs.push(("page", page.to_string()));
                        request.send()
                    })
                }
            }
        }
    };
    (@stream $verb:ident $method:ident $resp:ty) => {};
}

/// a detail endpoint with compile-time append_to_response: each `with_*`
/// fills one slot of the response type, and a requested-but-missing payload
/// is an error rather than a silent None. an append slot named differently
/// from its TMDB key declares `as "the/key"`. field attributes (serde) apply
/// to the private shadow struct that deserialization goes through; on the
/// public struct they're stripped via cfg_attr, since it has no Deserialize
/// derive
macro_rules! details {
    // no params section
    (
        $(#[$meta:meta])*
        $method:ident ($($pn:ident : $pt:ty),* $(,)?): GET $path:literal => $resp:ident {
            base { $($(#[$bm:meta])* pub $bf:ident : $bt:ty),* $(,)? }
            appends { $($(#[$am:meta])* $an:ident : $at:ty $(as $key:literal)?),* $(,)? }
        }
    ) => {
        details! {
            $(#[$meta])*
            $method($($pn : $pt),*): GET $path => $resp {
                params {}
                base { $($(#[$bm])* pub $bf : $bt),* }
                appends { $($(#[$am])* $an : $at $(as $key)?),* }
            }
        }
    };
    // full form
    (
        $(#[$meta:meta])*
        $method:ident ($($pn:ident : $pt:ty),* $(,)?): GET $path:literal => $resp:ident {
            params { $($qp:ident : $qt:ty),* $(,)? }
            base { $($(#[$bm:meta])* pub $bf:ident : $bt:ty),* $(,)? }
            appends { $($(#[$am:meta])* $an:ident : $at:ty $(as $key:literal)?),* $(,)? }
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
                path: ::std::string::String,
                pairs: ::std::vec::Vec<(&'static str, ::std::string::String)>,
                _appends: ::std::marker::PhantomData<fn() -> ($([<A $an:camel>]),*)>,
            }

            impl $crate::Client {
                $(#[$meta])*
                pub fn $method(&self, $($pn: $pt),*) -> [<$resp Request>] {
                    [<$resp Request>] {
                        client: self.clone(),
                        path: ::std::format!($path $(, $pn)*),
                        pairs: ::std::vec::Vec::new(),
                        _appends: ::std::marker::PhantomData,
                    }
                }
            }

            impl<$([<A $an:camel>]),*> [<$resp Request>]<$([<A $an:camel>]),*> {
                $(
                    pub fn $qp(mut self, value: $qt) -> Self {
                        self.pairs.push((stringify!($qp), $crate::ToParam::to_param(&value)));
                        self
                    }
                )*
            }

            details!(@withs [<$resp Request>] [$($pn,)*] [] [$($(#[$am])* $an : $at $(as $key)?,)*]);

            impl<$([<A $an:camel>]: $crate::Append),*> [<$resp Request>]<$([<A $an:camel>]),*> {
                pub async fn send(self) -> $crate::Result<$resp<$([<A $an:camel>]),*>> {
                    let mut pairs = self.pairs;
                    let mut append = ::std::string::String::new();
                    $(
                        if ![<A $an:camel>]::ABSENT {
                            append.push_str(details!(@key $an $(as $key)?));
                            append.push(',');
                        }
                    )*
                    if !append.is_empty() {
                        append.pop();
                        pairs.push(("append_to_response", append));
                    }
                    $resp::<$([<A $an:camel>]),*>::fetch(&self.client, &self.path, &pairs).await
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
                            $($an: [<A $an:camel>]::from_json(details!(@key $an $(as $key)?), shadow.$an)?,)*
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
                            $($an: <() as $crate::Append>::from_json(details!(@key $an $(as $key)?), shadow.$an)
                                .map_err(::serde::de::Error::custom)?,)*
                        })
                    }
                }
            };
        }
    };

    // the TMDB key for an append slot: the field name, or the `as` override
    (@key $an:ident) => { stringify!($an) };
    (@key $an:ident as $key:literal) => { $key };

    // one with_* per append slot, available only while that slot is ()
    (@withs $req:ident [$($pn:ident,)*] [$($bn:ident : $bt:ty,)*] []) => {};
    (@withs $req:ident [$($pn:ident,)*] [$($bn:ident : $bt:ty,)*] [$(#[$am:meta])* $an:ident : $at:ty $(as $key:literal)?, $($(#[$rm:meta])* $rest:ident : $rt:ty $(as $rkey:literal)?,)*]) => {
        ::paste::paste! {
            impl<$([<A $bn:camel>],)* $([<A $rest:camel>],)*> $req<$([<A $bn:camel>],)* (), $([<A $rest:camel>],)*> {
                $(#[$am])*
                #[doc = concat!("append `", details!(@key $an $(as $key)?), "` to the response")]
                pub fn [<with_ $an>](self) -> $req<$([<A $bn:camel>],)* $at, $([<A $rest:camel>],)*> {
                    $req {
                        client: self.client,
                        path: self.path,
                        pairs: self.pairs,
                        _appends: ::std::marker::PhantomData,
                    }
                }
            }
        }
        details!(@withs $req [$($pn,)*] [$($bn : $bt,)* $an : $at,] [$($(#[$rm])* $rest : $rt $(as $rkey)?,)*]);
    };
}
