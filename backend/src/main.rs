use actix_web::{get,web, App, HttpResponse, HttpServer, Responder, HttpRequest};
use tracing::instrument;

use crate::{app::AppState, db::NewUser};

mod db;
mod app;
mod logging;
mod errors;


#[get("/")]
#[instrument(skip_all,name="Index page",fields(uri = %req.uri(), method= %req.method()))]
pub async fn index(req: HttpRequest) -> Result<HttpResponse,errors::Error> {
    Ok(
        HttpResponse::Ok().body("Helo")
    )
}

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

    let app_state=web::Data::new(app_state);

    tracing::info!("Server started on 127.0.0.1:8000");
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(index)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}