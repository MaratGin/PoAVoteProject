use serde::{Serialize, Deserialize};
use crate::transaction::Transaction;


#[derive(Debug, Serialize, Deserialize)]
pub struct BlockHeader {
    pub previous_block_hash: String,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pub header: BlockHeader,
    pub hash: String,
    pub validator_public_key: String,
}

// impl Block {
//     pub fn new( previous_block_hash: String, validator_signature: String) -> Self {
//         let transactions_hash = Self::calculate_hash(&transactions);
//         let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
//         let header = BlockHeader {
//             timestamp,
//             nonce: 0,
//             previous_block_hash,
//             transactions_hash,
//             validator_signature,
//         };
//         Block { header, transactions }
//     }

//     pub fn calculate_hash<T: Serialize>(data: &T) -> String {
//         let serialized = serde_json::to_string(data).unwrap();
//         let mut hasher = Sha256::new();
//         hasher.update(serialized);
//         format!("{:x}", hasher.finalize())
//     }
// }



/*
#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    header: BlockHeader,
    timestamp: u128,
    prev_block_hash: String,
    transactions: Vec<Transaction>,
    hash: String,
    height: usize,
}

impl Block {
    pub fn new(transactions: Vec<Transaction>, previous_block_hash: String, validator_signature: String) -> Self {
        let transactions_hash = Block::calculate_hash(&transactions);
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let header = BlockHeader {
            timestamp,
            nonce: 0, // nonce может быть использован для майнинга, если это необходимо
            previous_block_hash,
            transactions_hash,
            validator_signature,
        };
        Block { header, transactions,timestamp,prev_block_hash, hash, height}
    }

    fn calculate_hash<T: Serialize>(data: &T) -> String {
        let serialized = serde_json::to_string(data).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(serialized);
        format!("{:x}", hasher.finalize())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockHeader {
    timestamp: u64,
    nonce: u64,
    previous_block_hash: String,
    transactions_hash: String,
    validator_signature: String, // Подпись валидатора (здесь просто как строка)
}



impl Blockchain {
    fn new() -> Blockchain {
        Blockchain { blocks: vec![] }
    }

    fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }
}
*/