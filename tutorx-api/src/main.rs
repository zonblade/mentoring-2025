mod services;

use std::sync::{Arc, Mutex};

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    unsafe{std::env::set_var("RUST_LOG", "debug");}
    env_logger::init();
    // database connection
    // middleware temporary data
    // static config yang tidak perlu LazyLock/lazy_static
    // MPSC producer shared state
    // dll...
    let connection = sqlite::open(":memory:").unwrap();

    let query = "
        CREATE TABLE rooms (name TEXT, number INTEGER);
        INSERT INTO rooms VALUES ('RoomX', 42);
    ";
    connection.execute(query).unwrap();

    let db_connection = Arc::new(Mutex::new(connection));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_connection.clone()))
            .route("/hello", web::get().to(services::rooms::detail::handler))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}