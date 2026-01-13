use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use sqlx::SqlitePool;
use crate::models::transaction::Transaction;

#[get("/transactions")]
pub async fn get_transactions(pool: web::Data<SqlitePool>) -> impl Responder {
    let res = sqlx::query_as::<_, Transaction>("SELECT * FROM transactions")
        .fetch_all(pool.get_ref())
        .await;

    match res {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/transactions")]
pub async fn create_transaction(
    pool: web::Data<SqlitePool>,
    item: web::Json<Transaction>,
) -> impl Responder {
    let res = sqlx::query(
        "INSERT INTO transactions (account_id, amount, transaction_type, description) VALUES (?, ?, ?, ?)"
    )
    .bind(item.account_id)
    .bind(item.amount)
    .bind(&item.transaction_type)
    .bind(&item.description)
    .execute(pool.get_ref())
    .await;


    match res {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[put("/transactions/{id}")]
pub async fn update_transaction(
    pool: web::Data<SqlitePool>,
    id: web::Path<i64>,
    item: web::Json<Transaction>,
) -> impl Responder {
    let res = sqlx::query(
        "UPDATE transactions SET account_id=?, amount=?, transaction_type=?, description=? WHERE id=?"
    )
    .bind(item.account_id)
    .bind(item.amount)
    .bind(&item.transaction_type)
    .bind(&item.description)
    .bind(*id)
    .execute(pool.get_ref())
    .await;

    match res {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[delete("/transactions/{id}")]
pub async fn delete_transaction(
    pool: web::Data<SqlitePool>,
    id: web::Path<i64>,
) -> impl Responder {
    let res = sqlx::query("DELETE FROM transactions WHERE id=?")
        .bind(*id)
        .execute(pool.get_ref())
        .await;

    match res {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
