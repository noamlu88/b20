use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use sqlx::SqlitePool;
use crate::models::customer::Customer;

#[get("/customers")]
pub async fn get_customers(pool: web::Data<SqlitePool>) -> impl Responder {
    let result = sqlx::query_as::<_, Customer>(
        "SELECT * FROM customers"
    )
    .fetch_all(pool.get_ref())
    .await;

    match result {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/customers")]
pub async fn create_customer(
    pool: web::Data<SqlitePool>,
    item: web::Json<Customer>,
) -> impl Responder {
    let res = sqlx::query(
        "INSERT INTO customers (full_name, email, phone) VALUES (?, ?, ?)"
    )
    .bind(&item.full_name)
    .bind(&item.email)
    .bind(&item.phone)
    .execute(pool.get_ref())
    .await;

    match res {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[put("/customers/{id}")]
pub async fn update_customer(
    pool: web::Data<SqlitePool>,
    id: web::Path<i64>,
    item: web::Json<Customer>,
) -> impl Responder {
    let res = sqlx::query(
        "UPDATE customers SET full_name=?, email=?, phone=? WHERE id=?"
    )
    .bind(&item.full_name)
    .bind(&item.email)
    .bind(&item.phone)
    .bind(*id)
    .execute(pool.get_ref())
    .await;

    match res {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[delete("/customers/{id}")]
pub async fn delete_customer(
    pool: web::Data<SqlitePool>,
    id: web::Path<i64>,
) -> impl Responder {
    let res = sqlx::query("DELETE FROM customers WHERE id=?")
        .bind(*id)
        .execute(pool.get_ref())
        .await;

    match res {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
