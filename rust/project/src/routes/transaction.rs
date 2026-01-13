use actix_web::web;
use crate::controllers::transaction::*;

pub fn transaction_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_transactions)
       .service(create_transaction)
       .service(update_transaction)
       .service(delete_transaction);
}
