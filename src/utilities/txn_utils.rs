use serde::{Deserialize, Serialize};
use serde_json::json;
use solana_sdk::sysvar::recent_blockhashes;
use solana_transaction_status::{
    EncodedConfirmedBlock, EncodedTransaction, EncodedTransactionWithStatusMeta, UiMessage, UiParsedMessage, UiRawMessage, UiTransaction
};

use std::str::FromStr;


#[derive(Debug, serde::Deserialize)]
pub struct ReducedTransaction {
    pub signatures: Vec<String>,
    pub recent_blockhash: String,
    pub account_keys: Vec<String>,
}

impl ReducedTransaction {
    pub fn new(
        signatures: Vec<String>,
        recent_blockhash: String,
        account_keys: Vec<String>,
    ) -> Self {
        Self {
            signatures,
            recent_blockhash,
            account_keys,
        }
    }
}

pub fn reduce_transaction(tx: &EncodedTransactionWithStatusMeta) -> (Vec<String>, String, Vec<String>) {
    match &tx.transaction {
        EncodedTransaction::Json(transaction) => {
            let mut signatures = Vec::new();
            for signature in &transaction.signatures {
                signatures.push(signature.to_string());
            }
            let recent_blockhash;
            let mut account_keys = Vec::new();
            match &transaction.message {
                UiMessage::Parsed(message) => {
                    recent_blockhash = message.recent_blockhash.to_string();
                    for account in &message.account_keys {
                        account_keys.push(account.pubkey.to_string());
                    }
                }
                UiMessage::Raw(message) => {
                    recent_blockhash = message.recent_blockhash.to_string();
                    for account in &message.account_keys {
                        account_keys.push(account.to_string());
                    }
                }
            }
            (signatures, recent_blockhash, account_keys)
        }
        _ => {
            let signatures = Vec::new();
            let recent_blockhashes = String::new();
            let account_keys = Vec::new();
            (signatures, recent_blockhashes, account_keys)
        }
    }
}

pub fn get_transactions(block: &EncodedConfirmedBlock) -> Vec<ReducedTransaction> {
    let mut transactions = Vec::new();
    for tx in &block.transactions {
        let (signatures, recent_blockhashes, account_keys) = reduce_transaction(tx);
        transactions.push(ReducedTransaction::new(signatures, recent_blockhashes, account_keys));
    }
    transactions
}

