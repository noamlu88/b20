use actix_web::web;
use crate::controllers::customer::*;

pub fn customer_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_customers)
       .service(create_customer)
       .service(update_customer)
       .service(delete_customer);
}
