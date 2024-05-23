use actix_web::{
    web::{Data, Json},
    HttpResponse, Responder,
};
use serde_json::json;

use crate::{
    helpers::{encrypt::verify_password, jwt::create_token},
    schemas::auth_schema::AuthenticateUserSchema,
    services::user_service::UserService,
    AppState,
};

pub async fn authenticate_user(
    body: Json<AuthenticateUserSchema>,
    data: Data<AppState>,
) -> impl Responder {
    let db = &data.db;

    let exists = match UserService::get_user_by_email(&db, &body.email).await {
        Ok(user) => user,
        Err(e) => {
            eprintln!("Failed to check if a user already exists: {}", e);
            return HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": format!("Failed to check if a user already exists: {:?}", e)
            }));
        }
    };

    if exists.is_none() {
        return HttpResponse::Unauthorized().json(json!({
            "status": "error",
            "message": "Invalid email"
        }));
    }

    let user = exists.unwrap();
    let passwords_matches = verify_password(&body.password, &user.password);

    if !passwords_matches {
        return HttpResponse::Unauthorized().json(json!({
            "status": "error",
            "message": "Invalid password"
        }));
    }

    let token = create_token(user.id.to_string());

    HttpResponse::Ok().json(json!({
        "status": "ok",
        "data": {
            "token": token,
        }
    }))
}
