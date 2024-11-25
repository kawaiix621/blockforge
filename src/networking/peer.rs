use std::collections::HashSet;

pub struct Peer {
    pub address: String,
    pub connected_peers: HashSet<String>,
}

impl Peer {
    pub fn new(address: String) -> Self {
        Peer {
            address,
            connected_peers: HashSet::new(),
        }
    }

    pub fn connect(&mut self, peer_address: String) {
        self.connected_peers.insert(peer_address);
    }

    pub fn disconnect(&mut self, peer_address: &String) {
        self.connected_peers.remove(peer_address);
    }
}
