//! Generic JSON adapters. Fixed-field Serde derives reject duplicate fields;
//! this module never parses into a map that could overwrite a duplicate key.

use serde::de::value::{MapAccessDeserializer, StringDeserializer};
use serde::de::{DeserializeOwned, MapAccess, Visitor};
use serde::{Deserialize, Deserializer};
use std::{fmt, marker::PhantomData};

pub(crate) fn decode<T: DeserializeOwned>(text: &str) -> Result<T, String> {
    decode_object(text).map_err(|error| error.to_string())
}

fn decode_object<T: DeserializeOwned>(text: &str) -> Result<T, serde_json::Error> {
    let mut decoder = serde_json::Deserializer::from_str(text);
    let value = object(&mut decoder)?;
    decoder.end()?;
    Ok(value)
}

pub(crate) fn object<'de, D, T>(decoder: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    decoder.deserialize_map(ObjectVisitor(PhantomData))
}

struct ObjectVisitor<T>(PhantomData<T>);

pub(crate) fn string_enum<'de, D, T>(decoder: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    let value = String::deserialize(decoder)?;
    T::deserialize(StringDeserializer::<D::Error>::new(value))
}

impl<'de, T: Deserialize<'de>> Visitor<'de> for ObjectVisitor<T> {
    type Value = T;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a JSON object")
    }

    fn visit_map<M: MapAccess<'de>>(self, map: M) -> Result<T, M::Error> {
        T::deserialize(MapAccessDeserializer::new(map))
    }
}
