use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

async fn greet(req: HttpRequest) -> impl Responder {
    let greet_name = req.match_info().get("name").unwrap_or("world");
    format!("Hello {}!", greet_name)
}

async fn health_check(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/name", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
