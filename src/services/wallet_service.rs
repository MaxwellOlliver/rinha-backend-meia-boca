use crate::{entity::wallet, schemas::wallet_schema::CreateWalletSchema};
use migration::sea_orm::{DatabaseConnection, DbErr};
use sea_orm::ActiveModelTrait;

pub struct WalletService;

impl WalletService {
    pub async fn create_wallet(
        db: &DatabaseConnection,
        form_data: CreateWalletSchema,
    ) -> Result<wallet::ActiveModel, DbErr> {
        let model = wallet::ActiveModel {
            user_id: sea_orm::ActiveValue::Set(form_data.user_id),
            ..Default::default()
        };

        model.save(db).await
    }
}
