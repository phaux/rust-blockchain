use std::{convert::TryFrom, fmt::Display};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "BlockchainData", into = "BlockchainData")]
pub struct Blockchain {
    pub blocks: Vec<Block>,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub payload: String,
}

impl Blockchain {
    pub fn new() -> Self {
        Self { blocks: Vec::new() }
    }

    pub fn anchor(&mut self, block: Block) {
        self.blocks.push(block);
    }
}

impl Block {
    pub fn new(payload: &str) -> Self {
        Self {
            payload: payload.to_owned(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct BlockchainData {
    blocks: Vec<BlockData>,
}

#[derive(Debug, Serialize, Deserialize)]
struct BlockData {
    #[serde(with = "digest::opt")]
    prev: Option<[u8; 32]>,
    payload: String,
    #[serde(with = "digest")]
    digest: [u8; 32],
}

/// Computes digests when serializing
impl Into<BlockchainData> for Blockchain {
    fn into(self) -> BlockchainData {
        let mut blocks = Vec::new();
        let mut prev_hash = None;
        for block in self.blocks {
            let current_hash = get_hash(prev_hash, &block.payload);
            blocks.push(BlockData {
                prev: prev_hash,
                payload: block.payload,
                digest: current_hash,
            });
            prev_hash = Some(current_hash);
        }
        BlockchainData { blocks }
    }
}

/// Checks digests when deserializing
impl TryFrom<BlockchainData> for Blockchain {
    type Error = Error;

    fn try_from(data: BlockchainData) -> Result<Self, Self::Error> {
        let mut prev_hash = None;
        let mut blocks = Vec::new();
        for block in data.blocks {
            if block.prev != prev_hash {
                return Err(Error::InvalidParentDigest);
            }
            let current_hash = get_hash(block.prev, &block.payload);
            if block.digest != current_hash {
                return Err(Error::InvalidDigest);
            }
            blocks.push(Block {
                payload: block.payload,
            });
            prev_hash = Some(current_hash);
        }
        Ok(Blockchain { blocks })
    }
}

#[derive(Debug)]
enum Error {
    InvalidDigest,
    InvalidParentDigest,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &Error::InvalidDigest => write!(f, "invalid digest"),
            &Error::InvalidParentDigest => write!(f, "invalid parent digest"),
        }
    }
}

impl std::error::Error for Error {}

fn get_hash(prev: Option<[u8; 32]>, payload: &str) -> [u8; 32] {
    #[derive(Debug, Serialize)]
    struct HashInput<'a> {
        prev: Option<[u8; 32]>,
        payload: &'a str,
    }

    let input = HashInput { prev, payload };
    let bytes = bincode::serialize(&input).unwrap();
    blake3::hash(&bytes).into()
}

// Serialize/deserialize base64 digest into/from byte array
mod digest {

    use serde::{de::Error, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(v: &[u8; 32], s: S) -> Result<S::Ok, S::Error> {
        opt::serialize(&Some(*v), s)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<[u8; 32], D::Error> {
        Ok(opt::deserialize(d)?.ok_or(Error::custom("missing digest"))?)
    }

    pub mod opt {
        use std::convert::TryInto;

        use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

        pub fn serialize<S: Serializer>(v: &Option<[u8; 32]>, s: S) -> Result<S::Ok, S::Error> {
            let base64 = v.map(base64::encode);
            base64.serialize(s)
        }

        pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Option<[u8; 32]>, D::Error> {
            let str = match <Option<String>>::deserialize(d)? {
                Some(v) => v,
                None => return Ok(None),
            };
            let vec = base64::decode(str).map_err(Error::custom)?;
            let digest = vec
                .try_into()
                .map_err(|_| Error::custom("invalid digest length"))?;
            Ok(Some(digest))
        }
    }
}
