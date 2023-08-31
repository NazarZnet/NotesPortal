pub mod handlers;
pub mod middleware;

use actix_web::web;
pub use middleware::*;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(handlers::signup_user)
        .service(handlers::login_user)
        .service(handlers::logout_handler)
        .service(handlers::refresh_auth);
}
