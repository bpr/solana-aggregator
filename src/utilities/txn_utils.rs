use solana_transaction_status::{ EncodedTransaction, EncodedTransactionWithStatusMeta, UiMessage };

#[allow(dead_code)]
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

#[allow(dead_code)]
pub fn contains_signature(txn: &EncodedTransactionWithStatusMeta, signature: &str) -> bool {
    match &txn.transaction {
        EncodedTransaction::Json(transaction) =>
            transaction.signatures.iter().any(|sig| sig == signature),
        EncodedTransaction::Accounts(transaction) =>
            transaction.signatures.iter().any(|sig| sig == signature),
        _ => false
    }
}

