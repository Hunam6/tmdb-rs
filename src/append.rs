use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::{Error, Page, Result};

/// one append_to_response slot in a detail response: `()` when the payload was
/// not requested, the payload type when it was
pub trait Append: Sized {
    const ABSENT: bool;
    fn from_json(slot: &'static str, value: Option<Value>) -> Result<Self>;
}

impl Append for () {
    const ABSENT: bool = true;

    fn from_json(_: &'static str, _: Option<Value>) -> Result<Self> {
        Ok(())
    }
}

impl<T: DeserializeOwned> Append for Page<T> {
    const ABSENT: bool = false;

    fn from_json(slot: &'static str, value: Option<Value>) -> Result<Self> {
        let value = value.ok_or(Error::MissingAppend(slot))?;
        Ok(serde_json::from_value(value)?)
    }
}

macro_rules! appendable {
    ($($t:ty),* $(,)?) => {$(
        impl $crate::Append for $t {
            const ABSENT: bool = false;

            fn from_json(slot: &'static str, value: Option<::serde_json::Value>) -> $crate::Result<Self> {
                let value = value.ok_or($crate::Error::MissingAppend(slot))?;
                Ok(::serde_json::from_value(value)?)
            }
        }
    )*};
}

pub(crate) use appendable;
