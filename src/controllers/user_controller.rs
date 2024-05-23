use actix_web::{
    web::{Data, Json},
    HttpResponse, Responder,
};
use serde_json::json;
use uuid::Uuid;

use crate::{
    schemas::user_schema::{CreateUserBodySchema, CreateUserSchema},
    services::user_service::UserService,
    AppState,
};

pub async fn create_user(body: Json<CreateUserBodySchema>, data: Data<AppState>) -> impl Responder {
    let db = &data.db;

    let exists = match UserService::get_user_by_email(&db, &body.email).await {
        Ok(user) => user.is_some(),
        Err(e) => {
            eprintln!("Failed to check if a user already exists: {}", e);
            return HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": format!("Failed to check if a user already exists: {:?}", e)
            }));
        }
    };

    if exists {
        return HttpResponse::Conflict().json(json!({
            "status": "error",
            "message": "A user with the same email already exists"
        }));
    }

    let user_id = Uuid::new_v4();

    let (user, wallet) = match UserService::create_user_and_wallet(
        &db,
        CreateUserSchema {
            id: user_id,
            email: body.email.clone(),
            name: body.name.clone(),
            password: body.password.clone(),
        },
    )
    .await
    {
        Ok((user, wallet)) => (user, wallet),
        Err(e) => {
            eprintln!("Failed to create user and wallet: {}", e);
            return HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": format!("Failed to create user and wallet: {:?}", e)
            }));
        }
    };

    HttpResponse::Created().json(json!({
        "status": "ok",
        "data": {
            "user": {
                "id": user.id,
                "email": user.email,
                "name": user.name,
            },
            "wallet": {
                "id": wallet.id,
                "balance": wallet.balance,
            },
        }
    }))
}
