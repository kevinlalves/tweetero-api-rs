use actix_web::{get, HttpResponse, Responder, web::ServiceConfig};

pub mod users;

#[get("/")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Ok")
}

pub fn app(cfg: &mut ServiceConfig) {
  cfg
    .service(health_check)
    .configure(users::routes);
}
