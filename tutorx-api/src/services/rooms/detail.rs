use actix_web::{web, HttpResponse, Responder};
use sqlite::Connection;
use std::sync::{Arc, Mutex};

pub async fn handler(db: web::Data<Arc<Mutex<Connection>>>)-> impl Responder {

    let query = "SELECT * FROM rooms WHERE number < 50";
    let db = db.as_ref().lock().unwrap();
    db.iterate(query, |pairs| {
            for &(name, value) in pairs.iter() {
                println!("{} = {}", name, value.unwrap());
            }
            true
        })
        .unwrap();

    HttpResponse::Ok().body("test")
}