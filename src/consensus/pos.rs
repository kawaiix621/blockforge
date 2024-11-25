pub struct ProofOfStake {
    stakes: Vec<(String, u64)>, // Vec<(address, stake_amount)>
}

impl ProofOfStake {
    pub fn new() -> Self {
        ProofOfStake { stakes: Vec::new() }
    }

    pub fn register_stake(&mut self, address: String, amount: u64) {
        self.stakes.push((address, amount));
    }

    pub fn select_validator(&self) -> Option<String> {
        if self.stakes.is_empty() {
            return None;
        }
        // Simple weighted random selection
        let total_stake: u64 = self.stakes.iter().map(|(_, amount)| amount).sum();
        let mut rng = rand::thread_rng();
        let roll = rand::Rng::gen_range(&mut rng, 0..total_stake);
        let mut cumulative = 0;
        for (address, stake) in &self.stakes {
            cumulative += stake;
            if roll < cumulative {
                return Some(address.clone());
            }
        }
        None
    }
}
