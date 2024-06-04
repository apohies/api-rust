use actix_web::{get, web, Responder};
use crate::main;

#[get("/flask")]
pub async fn flask() -> impl Responder {
    "Hello, World!"
}

#[get("sucher/{name}")]
pub async fn hello1(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", &name)
}


pub fn init_routes(cfg: &mut web::ServiceConfig) {

    cfg.service(
        web::scope("/api") // Asegúrate de incluir el scope "api" aquí
            .service(flask)
            .service(hello1)
    );
}