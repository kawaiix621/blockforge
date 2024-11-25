use std::collections::HashMap;

pub struct ByzantineFaultTolerance {
    nodes: Vec<String>,
    quorum: usize,
}

impl ByzantineFaultTolerance {
    pub fn new(nodes: Vec<String>) -> Self {
        let quorum = (2 * nodes.len()) / 3 + 1;
        ByzantineFaultTolerance { nodes, quorum }
    }

    pub fn reach_consensus(&self, votes: HashMap<String, bool>) -> bool {
        let positive_votes = votes.values().filter(|&&v| v).count();
        positive_votes >= self.quorum
    }
}
