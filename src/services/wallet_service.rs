use crate::{
    entity::{
        transaction::{self, TransactionType},
        wallet,
    },
    errors::ServiceError,
    schemas::{
        transaction_schema::CreateTransactionSchema,
        wallet_schema::{DepositSchema, TransferSchema, WithdrawalSchema},
    },
};
use migration::sea_orm::{DatabaseConnection, DbErr};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};

use super::transaction_service::TransactionService;

pub struct WalletService;

impl WalletService {
    pub async fn transfer(
        db: &DatabaseConnection,
        form_data: TransferSchema,
    ) -> Result<(wallet::Model, transaction::Model), ServiceError> {
        let user_id = &form_data.user_id;

        let recipient_wallet = WalletService::get_wallet_by_user_id(db, &form_data.recipient_id)
            .await
            .expect("Failed to get recipient wallet")
            .ok_or(DbErr::RecordNotFound(
                "Recipient wallet not found".to_string(),
            ))
            .unwrap();

        let sender_wallet = WalletService::get_wallet_by_user_id(db, user_id)
            .await
            .expect("Sender to get recipient wallet")
            .ok_or(DbErr::RecordNotFound("Sender wallet not found".to_string()))
            .unwrap();

        if sender_wallet.balance < form_data.amount {
            return Err(ServiceError::InsufficientBalance);
        }

        let sender_new_balance = sender_wallet.balance - form_data.amount;
        let recipient_new_balance = recipient_wallet.balance + form_data.amount;

        let mut sender_wallet: wallet::ActiveModel = sender_wallet.into();
        let mut recipient_wallet: wallet::ActiveModel = recipient_wallet.into();

        sender_wallet.balance = Set(sender_new_balance);
        recipient_wallet.balance = Set(recipient_new_balance);

        let sender_model = sender_wallet.update(db).await?;
        recipient_wallet.update(db).await?;

        let transaction_data = CreateTransactionSchema {
            user_id: *user_id,
            related_user_id: Some(form_data.recipient_id),
            amount: form_data.amount,
            transaction_type: TransactionType::Transfer,
        };

        let transaction = TransactionService::create_transaction(db, transaction_data)
            .await
            .map_err(ServiceError::DbError)?;

        Ok((sender_model, transaction))
    }

    pub async fn get_wallet_by_user_id(
        db: &DatabaseConnection,
        user_id: &uuid::Uuid,
    ) -> Result<Option<wallet::Model>, ServiceError> {
        wallet::Entity::find()
            .filter(wallet::Column::UserId.eq(*user_id))
            .one(db)
            .await
            .map_err(ServiceError::DbError)
    }

    pub async fn deposit(
        db: &DatabaseConnection,
        form_data: DepositSchema,
    ) -> Result<wallet::Model, ServiceError> {
        let wallet = WalletService::get_wallet_by_user_id(db, &form_data.user_id)
            .await?
            .unwrap();

        let model = wallet::ActiveModel {
            id: Set(wallet.id),
            user_id: Set(form_data.user_id),
            balance: Set(wallet.balance + form_data.amount),
            ..Default::default()
        };

        model.update(db).await.map_err(ServiceError::DbError)
    }

    pub async fn withdrawal(
        db: &DatabaseConnection,
        form_data: WithdrawalSchema,
    ) -> Result<(wallet::Model, transaction::Model), ServiceError> {
        let wallet = WalletService::get_wallet_by_user_id(db, &form_data.user_id).await?;

        let wallet = match wallet {
            Some(wallet) => wallet,
            None => return Err(ServiceError::WalletNotFound),
        };

        if (wallet.balance - form_data.amount) < 0 {
            return Err(ServiceError::InsufficientBalance);
        }

        let model = wallet::ActiveModel {
            id: Set(wallet.id),
            user_id: Set(form_data.user_id),
            balance: Set(wallet.balance - form_data.amount),
            ..Default::default()
        };

        let wallet_model = model.update(db).await.map_err(ServiceError::DbError)?;

        let transaction_data = CreateTransactionSchema {
            user_id: form_data.user_id,
            related_user_id: None,
            amount: form_data.amount,
            transaction_type: TransactionType::Withdrawal,
        };

        let transaction = TransactionService::create_transaction(db, transaction_data)
            .await
            .map_err(ServiceError::DbError)?;

        Ok((wallet_model, transaction))
    }
}
