use uuid::Uuid;

pub struct DepositSchema {
    pub user_id: Uuid,
    pub amount: i64,
}

pub struct WithdrawalSchema {
    pub user_id: Uuid,
    pub amount: i64,
}

pub struct TransferSchema {
    pub user_id: Uuid,
    pub recipient_id: Uuid,
    pub amount: i64,
}
