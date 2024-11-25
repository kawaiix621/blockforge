use blockforge::ledger::blockchain::{Block, Blockchain};

fn main() {
    let genesis_block = Block {
        index: 0,
        timestamp: "2024-11-24".to_string(),
        data: "Genesis Block".to_string(),
        previous_hash: "0".to_string(),
        hash: "GENESIS_HASH".to_string(),
        nonce: 0,
    };

    let mut blockchain = Blockchain::new(genesis_block);

    println!("Blockchain initialized with genesis block!");
}
