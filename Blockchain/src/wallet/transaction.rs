use std::error;

use crate::chain_util::ChainUtil;
pub use crate::wallet::transaction_input::TransactionInput;
pub use crate::wallet::transaction_output::TransactionOutput;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Transaction {
    pub id: String,
    pub tx_input: TransactionInput,
    pub tx_output: Vec<TransactionOutput>,
}

impl Transaction {
    pub fn new() -> Self {
        Transaction {
            id: ChainUtil::gen_id(),
            tx_output: vec![],
        }
    }

    pub fn transaction_with_output(
        sender_wallet: Wallet,
        tx_output: Vec<TransactionOutput>,
    ) -> Transaction {
        let transaction = Transaction::new();
        transaction.tx_output.extend(tx_output);
        Transaction.sign_transaction(transaction, sender_wallet);

        transaction
    }

    pub fn new_transaction(
        sender_wallet: Wallet,
        recipient: String,
        amount_to_send: u64,
    ) -> Result<Self, &'static str> {
        if amount_to_send > sender_wallet.balance {
            return Err("amount exceeds balance");
        }

        let tx_outputs: Vec<TransactionOutput> = vec![
            TransactionOutput {
                amount: sender_wallet.balance - amount_to_send,
                address: sender_wallet.public_key,
            },
            TransactionOutput {
                amount: amount_to_send,
                address: recipient,
            },
        ];

        Ok(Transaction::transaction_with_output(
            sender_wallet,
            tx_outputs,
        ))
    }

    pub fn new_reward_transaction(miner_wallet: Wallet, blockchain_wallet: Wallet) -> Transaction {
        let tx_outputs: Vec<TransactionOutput> = vec![
            TransactionOutput {
                amount: 99999,
                address: config.BLOCKCHAIN_WALLET_ADDRESS,
            },
            TransactionOutput {
                amount: config.MINING_REWARD,
                address: miner_wallet.public_key,
            },
        ];

        Transaction::transaction_with_output(miner_wallet, tx_outputs)
    }

    pub fn sign_transaction(transaction: Transaction, sender_wallet: Wallet) {
        transaction.tx_input = TransactionInput {
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
            amount:sender_wallet.balance,
            address:sender_wallet.public_key,
            signature: sender_wallet::sign(ChainUtil:::)
        }
    }
}
