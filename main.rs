use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::{SystemTime, UNIX_EPOCH};

// Block structure
#[derive(Debug)]
struct Block {
    index: u32,
    timestamp: u64,
    data: String,
    previous_hash: String,
    hash: String,
    nonce: u64,
}

// Blockchain structure
#[derive(Debug)]
struct Blockchain {
    chain: Vec<Block>,
    pending_transactions: Vec<String>,
    difficulty: usize,
}

impl Blockchain {
    /// Creates a new Blockchain with a genesis block.
    fn new(difficulty: usize) -> Self {
        let genesis_block = Block {
            index: 0,
            timestamp: 0,
            data: "Genesis Block".to_string(),
            previous_hash: "0".to_string(),
            hash: "0".to_string(),
            nonce: 0,
        };
        Blockchain {
            chain: vec![genesis_block],
            pending_transactions: vec![],
            difficulty,
        }
    }

    /// Adds a block to the blockchain with the given data.
    fn add_block(&mut self, data: String) {
        let previous_block = self.chain.last().expect("Blockchain should have at least one block.");
        let timestamp = current_timestamp();

        let (nonce, hash) = self.mine_block(previous_block.hash.clone(), timestamp, data.clone());

        let block = Block {
            index: self.chain.len() as u32,
            timestamp,
            data,
            previous_hash: previous_block.hash.clone(),
            hash,
            nonce,
        };
        self.chain.push(block);
    }

    /// Mines a block by finding a nonce that satisfies the difficulty condition.
    fn mine_block(&self, previous_hash: String, timestamp: u64, data: String) -> (u64, String) {
        let mut nonce = 0;
        loop {
            let hash = self.calculate_hash(&previous_hash, timestamp, &data, nonce);
            if self.is_valid_hash(&hash) {
                return (nonce, hash);
            }
            nonce += 1;
        }
    }

    /// Calculates the hash of the block using previous hash, timestamp, data, and nonce.
    fn calculate_hash(&self, previous_hash: &str, timestamp: u64, data: &str, nonce: u64) -> String {
        let mut hasher = DefaultHasher::new();
        previous_hash.hash(&mut hasher);
        timestamp.hash(&mut hasher);
        data.hash(&mut hasher);
        nonce.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }

    /// Validates that the hash meets the difficulty level by checking the number of leading zeros.
    fn is_valid_hash(&self, hash: &str) -> bool {
        hash.starts_with(&"0".repeat(self.difficulty))
    }

    /// Adds a transaction to the pending transactions list.
    fn add_transaction(&mut self, transaction: String) {
        self.pending_transactions.push(transaction);
    }

    /// Mines a new block with all pending transactions.
    fn mine_and_add_block(&mut self) {
        if self.pending_transactions.is_empty() {
            return;
        }
        let data = self.pending_transactions.join(";");
        self.add_block(data);
        self.pending_transactions.clear();
    }
}

/// Returns the current timestamp in seconds since UNIX_EPOCH.
fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}

fn main() {
    let mut blockchain = Blockchain::new(4); // Difficulty level set to 4

    blockchain.add_transaction("Transaction 1".to_string());
    blockchain.add_transaction("Transaction 2".to_string());
    blockchain.mine_and_add_block();

    blockchain.add_transaction("Transaction 3".to_string());
    blockchain.mine_and_add_block();

    println!("Blockchain:");
    for block in &blockchain.chain {
        println!(
            "Block {} - Hash: {} - Data: {} - Nonce: {}",
            block.index, block.hash, block.data, block.nonce
        );
    }
}
