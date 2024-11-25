use crate::crypto::hash::calculate_hash;

pub struct ProofOfWork {
    difficulty: usize,
}

impl ProofOfWork {
    pub fn new(difficulty: usize) -> Self {
        ProofOfWork { difficulty }
    }

    pub fn mine_block(&self, data: &str, previous_hash: &str) -> (String, u64) {
        let mut nonce = 0;
        loop {
            let hash = calculate_hash(&format!("{}{}{}", data, previous_hash, nonce));
            if hash.starts_with(&"0".repeat(self.difficulty)) {
                return (hash, nonce);
            }
            nonce += 1;
        }
    }
}
