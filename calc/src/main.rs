//web microservice for a calculator
use actix_web::{get, web, App, HttpServer, Responder};

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[get("/add/{a}/{b}")]
async fn add(info: web::Path<(i32, i32)>) -> impl Responder {
    let res = calc::add(info.0, info.1);
    format!("{} + {} = {}", info.0, info.1, res)
}

#[get("/subtract/{a}/{b}")]
async fn subtract(info: web::Path<(i32, i32)>) -> impl Responder {
    let res = calc::subtract(info.0, info.1);
    format!("{} - {} = {}", info.0, info.1, res)
}

#[get("/multiply/{a}/{b}")]
async fn multiply(info: web::Path<(i32, i32)>) -> impl Responder {
    let res = calc::multiply(info.0, info.1);
    format!("{} x {} = {}", info.0, info.1, res)
}

#[get("/divide/{a}/{b}")]
async fn divide(info: web::Path<(i32, i32)>) -> impl Responder {
    let res = calc::divide(info.0, info.1);
    format!("{} / {} = {}", info.0, info.1, res)
}


#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(greet)
            .service(add)
            .service(subtract)
            .service(multiply)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}