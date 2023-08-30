use actix_web::{get,web, App, HttpResponse, HttpServer, Responder};

mod db;
mod app;


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    //database setup
    let config = app::Settings::get_configuration().unwrap();
    let app_state = config
        .create_app_state()
        .expect("Failed to establish database connection");

    let app_state=web::Data::new(app_state);

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(hello)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}