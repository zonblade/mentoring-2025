use actix_web::{web, HttpResponse, Responder};
use sqlite::Connection;

pub fn handler(db: web::Data<Connection>)-> impl Responder {
    
    HttpResponse::Ok()
}