use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum, Deserialize, Serialize)]
#[sea_orm(rs_type = "String", db_type = "Text")]
pub enum TransactionType {
    #[sea_orm(string_value = "deposit")]
    Deposit,
    #[sea_orm(string_value = "withdrawal")]
    Withdrawal,
    #[sea_orm(string_value = "transfer")]
    Transfer,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "transactions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub user_id: Uuid,
    pub related_user_id: Option<Uuid>,
    pub amount: i64,
    pub transaction_type: TransactionType,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::RelatedUserId",
        to = "super::user::Column::Id"
    )]
    RelatedUser,
}

impl ActiveModelBehavior for ActiveModel {}
