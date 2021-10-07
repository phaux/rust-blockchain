use serde::{de, ser::SerializeStruct, Deserialize, Serialize};

use self::hash::{get_hash, Hash, HashInput};

mod hash;

#[derive(Debug)]
pub struct Blockchain {
    blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        Self { blocks: Vec::new() }
    }

    pub fn anchor(&mut self, block: Block) {
        self.blocks.push(block);
    }
}

impl Serialize for Blockchain {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut blocks = Vec::new();
        let mut prev_block: Option<HashInput> = None;

        for block in &self.blocks {
            let prev_hash = prev_block.as_ref().map(get_hash);
            let current_block = HashInput {
                prev: prev_hash,
                payload: &block.payload,
            };
            let current_hash = get_hash(&current_block);
            blocks.push(BlockData {
                prev: current_block.prev.clone(),
                payload: current_block.payload,
                digest: current_hash.clone(),
            });
            prev_block = Some(current_block);
        }

        let mut blockchain = serializer.serialize_struct("Blockchain", 1)?;
        blockchain.serialize_field("blocks", &blocks)?;
        blockchain.end()
    }
}

impl<'de> Deserialize<'de> for Blockchain {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        return deserializer.deserialize_map(BlockchainVisitor);

        struct BlockchainVisitor;

        impl<'de> de::Visitor<'de> for BlockchainVisitor {
            type Value = Blockchain;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("blockchain struct")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let blocks_data: Vec<BlockData> = match map.next_entry()? {
                    Some(("blocks", value)) => value,
                    _ => return Err(de::Error::missing_field("blocks")),
                };

                Ok(Blockchain {
                    blocks: blocks_data
                        .iter()
                        .map(|block_data| Block {
                            payload: block_data.payload.to_owned(),
                        })
                        .collect(),
                })
            }
        }
    }
}

/// Block struct in its serialized form.
#[derive(Debug, Serialize, Deserialize)]
struct BlockData<'a> {
    prev: Option<Hash>,
    payload: &'a str,
    digest: Hash,
}

#[derive(Debug)]
pub struct Block {
    payload: String,
}

impl Block {
    pub fn new(payload: &str) -> Self {
        Self {
            payload: payload.to_owned(),
        }
    }
}
