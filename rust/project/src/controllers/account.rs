use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use sqlx::SqlitePool;
use crate::models::account::Account;

#[get("/accounts")]
pub async fn get_accounts(pool: web::Data<SqlitePool>) -> impl Responder {
    let res = sqlx::query_as::<_, Account>("SELECT * FROM accounts")
        .fetch_all(pool.get_ref())
        .await;

    match res {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/accounts")]
pub async fn create_account(
    pool: web::Data<SqlitePool>,
    item: web::Json<Account>,
) -> impl Responder {
    let res = sqlx::query(
        "INSERT INTO accounts (customer_id, account_number, balance, account_type) VALUES (?, ?, ?, ?)"
    )
    .bind(item.customer_id)
    .bind(&item.account_number)
    .bind(item.balance)
    .bind(&item.account_type)
    .execute(pool.get_ref())
    .await;

    match res {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}


#[put("/accounts/{id}")]
pub async fn update_account(
    pool: web::Data<SqlitePool>,
    id: web::Path<i64>,
    item: web::Json<Account>,
) -> impl Responder {
    let res = sqlx::query(
        "UPDATE accounts SET account_number=?, balance=?, account_type=? WHERE id=?"
    )
    .bind(&item.account_number)
    .bind(item.balance)
    .bind(&item.account_type)
    .bind(*id)
    .execute(pool.get_ref())
    .await;

    match res {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[delete("/accounts/{id}")]
pub async fn delete_account(
    pool: web::Data<SqlitePool>,
    id: web::Path<i64>,
) -> impl Responder {
    let res = sqlx::query("DELETE FROM accounts WHERE id=?")
        .bind(*id)
        .execute(pool.get_ref())
        .await;

    match res {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

