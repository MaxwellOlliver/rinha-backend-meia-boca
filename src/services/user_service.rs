use crate::{entity::user, helpers::encrypt, schemas::user_schema::CreateUserSchema};
use actix_web::web::Json;
use migration::sea_orm::{ColumnTrait, DatabaseConnection, DbErr};
use sea_orm::{ActiveModelTrait, EntityTrait, QueryFilter};

pub struct UserService;

impl UserService {
    pub async fn create_user(
        db: &DatabaseConnection,
        form_data: &Json<CreateUserSchema>,
    ) -> Result<user::ActiveModel, DbErr> {
        let hashed_password = encrypt::hash_password(&form_data.password);

        let model = user::ActiveModel {
            email: sea_orm::ActiveValue::Set(form_data.email.to_string()),
            name: sea_orm::ActiveValue::Set(form_data.name.to_string()),
            password: sea_orm::ActiveValue::Set(hashed_password),
            ..Default::default()
        };

        model.save(db).await
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
