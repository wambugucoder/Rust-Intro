use actix_web::{Responder, App, HttpServer, web};

async fn welcome() -> impl Responder {
    format!("Hello world")
}

#[actix_rt::main]
async fn main()->std::io::Result<()> {
    HttpServer::new(||{
        App::new()
            .route("/",web::get().to(welcome()))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await

}