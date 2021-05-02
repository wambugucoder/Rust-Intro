mod Employees;

use actix_web::{Responder, App, HttpServer, web};

async fn welcome() -> impl Responder {
    format!("Hello world")
}

#[actix_rt::main]
async fn main()->std::io::Result<()> {
    HttpServer::new(||{
        App::new()
            .configure(Employees::init_routes())
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await

}