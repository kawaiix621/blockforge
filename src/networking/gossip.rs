pub fn broadcast_message(peers: &Vec<String>, message: &str) {
    for peer in peers {
        println!("Sending message '{}' to peer {}", message, peer);
    }
}
