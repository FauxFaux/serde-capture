use serde::de::Error;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde_derive::Deserialize;

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct CaptureJson<T> {
    pub inner: T,
    pub bytes: Box<[u8]>,
}

#[derive(Deserialize)]
pub struct Nothing {}

impl<T> CaptureJson<T> {
    pub fn into_inner(self) -> T {
        self.inner
    }

    pub fn into_bytes(self) -> Box<[u8]> {
        self.bytes
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for CaptureJson<T> {
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

        Ok(CaptureJson {
            bytes: bytes.into_boxed_slice(),
            inner: extracted,
        })
    }
}
