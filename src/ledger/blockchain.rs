pub struct Block {
    pub index: usize,
    pub timestamp: String,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new(genesis_block: Block) -> Self {
        Blockchain {
            chain: vec![genesis_block],
        }
    }

    pub fn add_block(&mut self, block: Block) {
        self.chain.push(block);
    }
}
