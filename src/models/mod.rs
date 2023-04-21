use diesel::prelude::Queryable;

#[derive(Queryable)]
pub struct User {
  pub id: String,
  pub username: String,
  pub password: String,
  pub avatar: String,
}
