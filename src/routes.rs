use actix_web::web::{self, resource, ServiceConfig};

use crate::{
    controllers::{
        auth_controller::authenticate_user,
        transaction_controller::list_user_transactions,
        user_controller::create_user,
        wallet_controller::{deposit_money, get_user_balance, transfer_money, withdrawal_money},
    },
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
    cfg.service(
        web::scope("/wallet")
            .wrap(Authentication)
            .route("/deposit", web::post().to(deposit_money))
            .route("/withdrawal", web::post().to(withdrawal_money))
            .route("/transfer", web::post().to(transfer_money))
            .route("/transactions", web::get().to(list_user_transactions))
            .route("/balance", web::get().to(get_user_balance)),
    );
}
