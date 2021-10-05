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
    dbg!(bc);
}
