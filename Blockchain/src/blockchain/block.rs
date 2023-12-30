use std::{
    simd::SimdFloat,
    time::{SystemTime, UNIX_EPOCH},
    u8,
};

use sha2::{
    digest::{crypto_common::KeyInit, FixedOutput, Update},
    Sha256,
};

use crate::wallet::{transaction, transaction_input};

// use super::blockchain::Blockchain;
// use crate::chain_util;

pub struct Block {
    pub time_stamp: u64,
    pub last_hash: String,
    pub hash: Vec<u8>,

    pub data: String,
    pub nonce: usize,
    pub difficulty: usize,
}

impl Block {
    fn get_genesis_block(&self) -> Block {
        let genesis_tx: Transaction;
        genesis_tx.id = "genesis";
        genesis_tx.tx_input = TransactionInput {
            amount: 0,
            address: "-----",
        };
        return Block {
            time_stamp: 0,
            last_hash: String::from("----"),
            hash: String::from("first_hash"),
            data: String::from("rajmadethisblock"),
            nonce: 0,
            difficulty: 1,
        };
    }

    pub fn mine_new_block(last_block: &Block, data: String) -> Block {
        let time_stamp: u64;
        let last_hash: String = last_block.hash;
        let mut nonce: usize = 0;
        let hash: Vec<u8>;
        let Block { difficulty, .. } = last_block;
        //proof of work

        loop {
            let time_stamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs();
            difficulty = Block::adjust_difficulty(last_block, time_stamp);
            nonce = nonce + 1;
            hash = Block::generate_hash(time_stamp, last_hash, data, nonce, difficulty);

            if hash.starts_with(&[0; 4]) {
                break;
            }
            nonce += 1;
        }
        return Block {
            time_stamp,
            last_hash,
            hash,
            data,
            nonce,
            difficulty,
        };
    }

    pub fn generate_hash(
        time_stamp: u64,
        last_hash: String,
        data: String,
        nonce: usize,
        difficulty: &usize,
    ) -> Result<Vec<u8>, std::io::Error> {
        let mut hasher = Sha256::new();
        hasher.update(&data);
        hasher.update(&time_stamp.to_be_bytes());
        hasher.update(&last_hash);
        hasher.update(&nonce.to_string());

        Ok(hasher.finalize_fixed().to_vec())
    }

    pub fn generate_hash2(&self) -> String {
        return String::from("hash");
    }
}
