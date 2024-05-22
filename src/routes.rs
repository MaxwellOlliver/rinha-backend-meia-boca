use actix_web::{get, web};

use crate::controllers::user_controller::create_user;

#[get("/ping")]
async fn ping() -> impl actix_web::Responder {
    "pong"
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(ping);
    cfg.service(web::scope("/api").service(create_user));
}
