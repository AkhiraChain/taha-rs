use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug)]
pub struct Tx {
    pub hash: String,
    pub block_number: u64,
    pub from: String,
    pub to: String,
    pub amount: i64,
    pub nonce: u64,
    pub timestamp: DateTime<Utc>,
    pub order: i32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Stats {
    pub txns: Vec<Tx>,
    pub total_amount: f64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    pub number: i64,
    pub hash: String,
    pub timestamp: DateTime<Utc>,
    pub tx_count: usize,
    pub txns: Vec<Tx>
}