use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub voter: String,
    pub choice: String,
    pub timestamp: u128,
}