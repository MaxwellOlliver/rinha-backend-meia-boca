use crate::{
    entity::{user, wallet},
    helpers::encrypt,
    schemas::{user_schema::CreateUserSchema, wallet_schema::CreateWalletSchema},
};
use migration::sea_orm::{ColumnTrait, DatabaseConnection, DbErr};
use sea_orm::{ActiveModelTrait, EntityTrait, QueryFilter, TransactionTrait};

pub struct UserService;

impl UserService {
    pub async fn create_user_and_wallet(
        db: &DatabaseConnection,
        user_data: CreateUserSchema,
    ) -> Result<(user::Model, wallet::Model), DbErr> {
        let hashed_password = encrypt::hash_password(&user_data.password);

        let txn = db
            .begin()
            .await
            .map_err(|e| {
                eprintln!("Failed to start transaction: {}", e);
                actix_web::error::ErrorInternalServerError(format!(
                    "Failed to start transaction: {}",
                    e
                ))
            })
            .expect("Failed to start transaction");

        let new_user = user::ActiveModel {
            id: sea_orm::ActiveValue::Set(user_data.id),
            email: sea_orm::ActiveValue::Set(user_data.email.to_string()),
            name: sea_orm::ActiveValue::Set(user_data.name.to_string()),
            password: sea_orm::ActiveValue::Set(hashed_password),
            ..Default::default()
        };

        let user = new_user
            .insert(&txn)
            .await
            .map_err(|e| {
                eprintln!("Failed to insert user: {}", e);
                actix_web::error::ErrorInternalServerError(format!("Failed to insert user: {}", e))
            })
            .expect("Failed to create user");

        let new_wallet = wallet::ActiveModel {
            user_id: sea_orm::ActiveValue::Set(user.id.clone()),
            ..Default::default() // Certificando que não estamos tentando atualizar um registro existente
        };

        let wallet = new_wallet
            .insert(&txn)
            .await
            .map_err(|e| {
                eprintln!("Failed to insert wallet: {}", e);
                actix_web::error::ErrorInternalServerError(format!(
                    "Failed to insert wallet: {}",
                    e
                ))
            })
            .expect("Failed to create wallet");

        if let Err(e) = txn.commit().await {
            eprintln!("Failed to commit transaction: {}", e);
            return Err(DbErr::from(e));
        }

        Ok((user, wallet))
    }

    pub async fn get_user_by_email(
        db: &DatabaseConnection,
        email: &String,
    ) -> Result<Option<user::Model>, DbErr> {
        user::Entity::find()
            .filter(user::Column::Email.contains(email))
            .one(db)
            .await
    }
}
