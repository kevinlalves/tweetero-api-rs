use actix_web::web::{ServiceConfig, self};

use crate::controllers::users::create;

pub fn routes(cfg: &mut ServiceConfig) {
  cfg.service(
    web::scope("/users")
      .service(create)
  );
}
