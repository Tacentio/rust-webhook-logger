use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, HttpRequest};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/ping")]
async fn echo() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/webhook")]
async fn webhook(req: HttpRequest) -> impl Responder {
    println!("{:?}", req);
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(echo)
            .service(webhook)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
