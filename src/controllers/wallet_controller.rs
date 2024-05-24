use actix_web::{
    web::{self, Data},
    HttpMessage, HttpRequest, HttpResponse, Responder,
};
use serde_json::json;
use uuid::Uuid;

use crate::{
    entity::transaction::{self, TransactionType},
    errors::ServiceError,
    schemas::{
        transaction_schema::{
            CreateTransactionSchema, DepositRequest, TransferRequest, WithdrawalRequest,
        },
        wallet_schema::{DepositSchema, TransferSchema, WithdrawalSchema},
    },
    services::{transaction_service::TransactionService, wallet_service::WalletService},
    AppState,
};

fn map_service_error(error: ServiceError) -> HttpResponse {
    match error {
        ServiceError::InsufficientBalance => HttpResponse::BadRequest().json(json!({
            "status": "error",
            "message": "Insufficient balance"
        })),
        ServiceError::WalletNotFound => HttpResponse::NotFound().json(json!({
            "status": "error",
            "message": "Wallet not found"
        })),
        ServiceError::DbError(e) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": format!("Database error: {:?}", e)
        })),
    }
}

pub async fn get_user_balance(data: Data<AppState>, req: HttpRequest) -> impl Responder {
    let extensions = req.extensions();
    let sub = extensions.get::<String>().unwrap();
    let user_id = Uuid::parse_str(sub).unwrap();
    let db = &data.db;

    let wallet = match WalletService::get_wallet_by_user_id(&db, &user_id).await {
        Ok(wallet) => match wallet {
            Some(wallet) => wallet,
            None => {
                return map_service_error(ServiceError::WalletNotFound);
            }
        },
        Err(e) => {
            eprintln!("Failed to get wallet: {}", e);
            return map_service_error(e);
        }
    };

    HttpResponse::Ok().json(json!({
        "status": "ok",
        "data": {
            "balance": wallet.balance,
        }
    }))
}

pub async fn deposit_money(
    req: HttpRequest,
    body: web::Json<DepositRequest>,
    data: web::Data<AppState>,
) -> impl Responder {
    let extensions = req.extensions();
    let sub = extensions.get::<String>().unwrap();
    let user_id = Uuid::parse_str(sub).unwrap();
    let db = &data.db;

    let deposit_data = DepositSchema {
        user_id,
        amount: body.amount,
    };

    match WalletService::deposit(db, deposit_data).await {
        Ok(wallet) => {
            let transaction_data = CreateTransactionSchema {
                user_id,
                related_user_id: None,
                amount: body.amount,
                transaction_type: TransactionType::Deposit,
            };

            let transaction = TransactionService::create_transaction(db, transaction_data)
                .await
                .expect("Failed to create transaction");

            HttpResponse::Ok().json(json!({
                "status": "ok",
                "data": {
                    "transaction": transaction,
                    "balance": wallet.balance,
                }
            }))
        }
        Err(e) => {
            eprintln!("Failed to deposit: {}", e);
            return map_service_error(e);
        }
    }
}

pub async fn transfer_money(
    req: HttpRequest,
    body: web::Json<TransferRequest>,
    data: web::Data<AppState>,
) -> impl Responder {
    let extensions = req.extensions();
    let sub = extensions.get::<String>().unwrap();
    let user_id = Uuid::parse_str(sub).unwrap();
    let db = &data.db;

    let form_data = TransferSchema {
        user_id,
        recipient_id: body.recipient_id,
        amount: body.amount,
    };

    let (sender_wallet, transaction) = match WalletService::transfer(db, form_data).await {
        Ok((sender_wallet, transaction)) => (sender_wallet, transaction),
        Err(e) => {
            eprintln!("Failed to transfer: {}", e);
            return map_service_error(e);
        }
    };

    HttpResponse::Ok().json(json!({
        "status": "ok",
        "data": {
            "transaction": transaction,
            "balance": sender_wallet.balance,
        }
    }))
}

pub async fn withdrawal_money(
    data: Data<AppState>,
    req: HttpRequest,
    body: web::Json<WithdrawalRequest>,
) -> impl Responder {
    let extensions = req.extensions();
    let sub = extensions.get::<String>().unwrap();
    let user_id = Uuid::parse_str(sub).unwrap();
    let db = &data.db;

    let form_data = WithdrawalSchema {
        user_id,
        amount: body.amount,
    };

    let (wallet, transaction) = match WalletService::withdrawal(&db, form_data).await {
        Ok((wallet, transaction)) => (wallet, transaction),
        Err(e) => {
            eprintln!("Failed to get wallet: {}", e);
            return map_service_error(e);
        }
    };

    HttpResponse::Ok().json(json!({
        "status": "ok",
        "data": {
            "balance": wallet.balance,
            "transaction": transaction,
        }
    }))
}
