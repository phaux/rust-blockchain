//! Adapted from https://users.rust-lang.org/t/serialize-a-vec-u8-to-json-as-base64/57781/5

use std::convert::TryInto;

use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone)]
pub(crate) struct Hash(pub [u8; 32]);

impl Serialize for Hash {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(&base64::encode(&self.0))
    }
}

impl<'de> Deserialize<'de> for Hash {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        return deserializer.deserialize_str(Base64Visitor);

        struct Base64Visitor;

        impl de::Visitor<'_> for Base64Visitor {
            type Value = Hash;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a base64 hash string")
            }

            fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
                let bytes = base64::decode(v).map_err(de::Error::custom)?;
                let hash: [u8; 32] = bytes
                    .try_into()
                    .map_err(|_| de::Error::custom("invalid hash length"))?;
                Ok(Hash(hash))
            }
        }
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct HashInput<'a> {
    pub prev: Option<Hash>,
    pub payload: &'a str,
}

pub(crate) fn get_hash(input: &HashInput) -> Hash {
    Hash(blake3::hash(&bincode::serialize(input).unwrap()).into())
}
