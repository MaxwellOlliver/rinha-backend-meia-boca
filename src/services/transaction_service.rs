use crate::{entity::transaction, schemas::transaction_schema::CreateTransactionSchema};
use migration::sea_orm::{DatabaseConnection, DbErr};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, QueryFilter, QueryOrder, QuerySelect,
};

pub struct TransactionService;

impl TransactionService {
    pub async fn create_transaction(
        db: &DatabaseConnection,
        form_data: CreateTransactionSchema,
    ) -> Result<transaction::Model, DbErr> {
        let model = transaction::ActiveModel {
            user_id: sea_orm::ActiveValue::Set(form_data.user_id),
            related_user_id: sea_orm::ActiveValue::Set(form_data.related_user_id),
            amount: sea_orm::ActiveValue::Set(form_data.amount),
            transaction_type: sea_orm::ActiveValue::Set(form_data.transaction_type),
            ..Default::default()
        };

        model.insert(db).await
    }

    pub async fn get_user_transactions(
        db: &DatabaseConnection,
        user_id: &uuid::Uuid,
        limit: u64,
        offset: u64,
    ) -> Result<Vec<transaction::Model>, DbErr> {
        let condition = Condition::any()
            .add(transaction::Column::UserId.eq(user_id.clone()))
            .add(transaction::Column::RelatedUserId.eq(user_id.clone()));

        let transactions = transaction::Entity::find()
            .filter(condition)
            .order_by_desc(transaction::Column::CreatedAt)
            .limit(limit)
            .offset(offset)
            .all(db)
            .await?;

        Ok(transactions)
    }
}
