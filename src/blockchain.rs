use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(from = "BlockchainData", into = "BlockchainData")]
pub struct Blockchain {
    blocks: Vec<Block>,
}

#[derive(Debug, Clone)]
pub struct Block {
    payload: String,
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

impl Into<BlockchainData> for Blockchain {
    fn into(self) -> BlockchainData {
        let mut blocks = Vec::new();
        let mut prev_block = None;
        for block in &self.blocks {
            let prev_hash = prev_block.as_ref().map(get_hash);
            let current_block = HashInput {
                prev: prev_hash,
                payload: &block.payload,
            };
            let current_hash = get_hash(&current_block);
            blocks.push(BlockData {
                prev: current_block.prev.clone(),
                payload: current_block.payload.to_owned(),
                digest: (current_hash),
            });
            prev_block = Some(current_block);
        }
        BlockchainData { blocks }
    }
}

impl From<BlockchainData> for Blockchain {
    fn from(blockchain: BlockchainData) -> Self {
        Self {
            blocks: blockchain
                .blocks
                .into_iter()
                .map(|block| Block {
                    payload: block.payload,
                })
                .collect(),
        }
    }
}

#[derive(Debug, Serialize)]
struct HashInput<'a> {
    prev: Option<[u8; 32]>,
    payload: &'a str,
}

fn get_hash(input: &HashInput) -> [u8; 32] {
    blake3::hash(&bincode::serialize(input).unwrap()).into()
}

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
