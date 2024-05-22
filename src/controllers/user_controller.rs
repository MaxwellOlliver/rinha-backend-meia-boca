use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse, Responder,
};
use serde_json::json;

use crate::{
    schemas::user_schema::CreateUserSchema, services::user_service::UserService, AppState,
};

#[post("/users")]
pub async fn create_user(body: Json<CreateUserSchema>, data: Data<AppState>) -> impl Responder {
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

    match UserService::create_user(&db, &body).await {
        Ok(_) => HttpResponse::Created().json(json!({
            "status": "ok",
        })),
        Err(e) => {
            eprintln!("Failed to create a user: {}", e);
            HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": format!("Failed to create a user: {:?}", e)
            }))
        }
    }
}
