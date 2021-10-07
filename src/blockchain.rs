use crate::hash::BlockData;
use serde_json::json;

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

    pub fn serialize(&self) -> String {
        let mut prev_block_data: Option<BlockData> = None;
        let mut json_blocks = Vec::new();

        for block in &self.blocks {
            let block_data = BlockData {
                parent_hash: prev_block_data.as_ref().map(|data| data.hash()),
                payload: &block.payload,
            };
            json_blocks.push(json!({
                "prev": block_data.parent_hash.as_ref().map(|hash| base64::encode(hash)),
                "payload": &block_data.payload,
                "digest": base64::encode(block_data.hash()),
            }));
            prev_block_data = Some(block_data);
        }

        json!({ "blocks": json_blocks }).to_string()
    }
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
