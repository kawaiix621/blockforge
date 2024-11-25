pub trait Consensus {
    fn validate_block(&self, block_data: &str, previous_hash: &str) -> bool;
}
