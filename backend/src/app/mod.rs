pub mod configuration;
pub mod handlers;

use actix_web::web;
pub use configuration::*;



pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
    .service(handlers::get_posts)
    .service(handlers::add_post)
    .service(handlers::update_posts);
}
