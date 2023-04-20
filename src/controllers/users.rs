use actix_web::{post, HttpResponse, Responder};

#[post("")]
pub async fn create() -> impl Responder {
  HttpResponse::Created()
}
