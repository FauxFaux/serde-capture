use std::fmt;
use std::marker::PhantomData;

use serde::de::Error;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;

pub struct Capture<T> {
    pub extracted: T,
    pub bytes: Box<[u8]>,
}

impl<T> Capture<T> {
    pub fn into_extracted(self) -> T {
        self.extracted
    }

    pub fn into_bytes(self) -> Box<[u8]> {
        self.bytes
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Capture<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let inner = serde_json::Value::deserialize(deserializer)?;

        let mut bytes = Vec::with_capacity(128);
        inner
            .serialize(&mut serde_json::Serializer::new(&mut bytes))
            .map_err(|e| D::Error::custom(format!("repacking failed: {:?}", e)))?;

        let extracted = T::deserialize(inner)
            .map_err(|e| D::Error::custom(format!("extraction failed: {:?}", e)))?;

        Ok(Capture {
            bytes: bytes.into_boxed_slice(),
            extracted,
        })
    }
}
