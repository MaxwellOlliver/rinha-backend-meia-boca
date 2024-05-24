use actix_web::{
    web::{self, Query},
    HttpMessage, HttpRequest, HttpResponse, Responder,
};
use serde_json::json;
use uuid::Uuid;

use crate::{
    schemas::transaction_schema::ListTransactionsQuery,
    services::transaction_service::TransactionService, AppState,
};
pub async fn list_user_transactions(
    req: HttpRequest,
    data: web::Data<AppState>,
    query: Query<ListTransactionsQuery>,
) -> impl Responder {
    let extensions = req.extensions();
    let sub = extensions.get::<String>().unwrap();
    let user_id = Uuid::parse_str(sub).unwrap();
    let db = &data.db;

    let limit = query.page_size.unwrap_or(10);
    let offset = (query.page.unwrap_or(1) - 1) * limit;

    match TransactionService::get_user_transactions(db, &user_id, limit, offset).await {
        Ok(transactions) => HttpResponse::Ok().json(json!({
            "status": "ok",
            "data": transactions
        })),
        Err(e) => {
            eprintln!("Failed to list transactions: {}", e);
            HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": format!("Failed to list transactions: {:?}", e)
            }))
        }
    }
}
