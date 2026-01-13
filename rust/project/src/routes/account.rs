use actix_web::web;
use crate::controllers::account::*;

pub fn account_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_accounts)
       .service(create_account)
       .service(update_account)
       .service(delete_account);
}
