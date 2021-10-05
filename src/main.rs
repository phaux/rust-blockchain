use serde_json::json;

mod hash;

#[derive(Debug)]
struct Blockchain {
    blocks: Vec<Block>,
}

impl Blockchain {
    fn new() -> Self {
        Self { blocks: Vec::new() }
    }

    fn anchor(&mut self, block: Block) {
        self.blocks.push(block);
    }

    fn serialize(&self) -> String {
        let mut prev_block_data: Option<hash::BlockData> = None;
        let mut json_blocks = Vec::new();

        for block in &self.blocks {
            let block_data = hash::BlockData {
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
struct Block {
    payload: String,
}

impl Block {
    fn new(payload: &str) -> Self {
        Self {
            payload: payload.to_owned(),
        }
    }
}

fn main() {
    let mut bc = Blockchain::new();
    bc.anchor(Block::new("block 1"));
    bc.anchor(Block::new("block 2"));
    println!("{}", bc.serialize());
}
