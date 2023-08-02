use actix_web::{get, web::Data, App, HttpServer};
mod todolist;
use todolist::{services, repository::MongoRepo};

#[get("/")]
async fn index() -> String {
    "Welcome to MongoDB Actix TodoApp".to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    
    let db = MongoRepo::init().await;
    let db_data = Data::new(db);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(index)
            .configure(services::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
