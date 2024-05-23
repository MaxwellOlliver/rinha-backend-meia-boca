use actix_web::web::{resource, ServiceConfig};

use crate::{
    controllers::{auth_controller::authenticate_user, user_controller::create_user},
    middlewares::auth_middleware::Authentication,
};

async fn ping() -> impl actix_web::Responder {
    "pong"
}

pub fn configure_public(cfg: &mut ServiceConfig) {
    cfg.service(resource("/ping").get(ping));

    cfg.service(resource("/users").post(create_user));
    cfg.service(resource("/auth").post(authenticate_user));
}

pub fn configure_protected(cfg: &mut ServiceConfig) {
    cfg.service(resource("/ping").wrap(Authentication).get(ping));
}
