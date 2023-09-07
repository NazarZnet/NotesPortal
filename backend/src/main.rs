mod app;
mod auth;
mod db;
mod errors;
mod logging;
mod schema;

use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger, web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let subscriber = logging::get_tracing_subscriber("debug", std::io::stdout);
    logging::init_tracing_subscriber(subscriber).unwrap_or_else(|e| tracing::error!(e));

    //database setup
    let config = app::Settings::get_configuration().unwrap();
    let app_state = config
        .create_app_state()
        .expect("Failed to establish database connection");

    let app_state = web::Data::new(app_state);

    tracing::info!("Server started on 127.0.0.1:8000");
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://127.0.0.1:8080")
            .supports_credentials()
            .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ]);
        App::new()
            .app_data(app_state.clone())
            .wrap(cors)
            .wrap(Logger::default())
            .configure(auth::config)
            .configure(app::config)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
