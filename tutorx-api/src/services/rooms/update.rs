use actix_web::{web, HttpResponse, Responder};

pub fn handler()-> impl Responder {
    HttpResponse::Ok()
}