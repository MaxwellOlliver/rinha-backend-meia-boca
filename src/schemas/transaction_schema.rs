use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entity::transaction::TransactionType;

#[derive(Debug, Serialize, Deserialize)]
pub struct DepositRequest {
    pub amount: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WithdrawalRequest {
    pub amount: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransferRequest {
    pub recipient_id: Uuid,
    pub amount: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListTransactionsQuery {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

pub struct CreateTransactionSchema {
    pub user_id: Uuid,
    pub related_user_id: Option<Uuid>,
    pub amount: i64,
    pub transaction_type: TransactionType,
}
