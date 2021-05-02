use actix_web::{Responder, HttpResponse,get,web};
use serde_json::json;
use crate::Employees::Users;
use actix_web::web::ServiceConfig;

#[get("/employees")]
async fn find_all()->impl Responder{
   HttpResponse::Ok().json(
      Users{
         id: 0,
         first_name: "abc".to_string(),
         last_name: "cde".to_string(),
         department: "none".to_string(),
         salary: 0,
         age: 0
      }
   )

}

pub fn init_routes(config:&mut web::ServiceConfig){
   config.service(find_all());

}