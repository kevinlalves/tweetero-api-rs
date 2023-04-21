use chrono::{DateTime, Local};
use diesel::{prelude::Queryable, Insertable};
use crate::schema::users;

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
  pub username: &'a str,
  pub password: &'a str,
  pub avatar: &'a str,
}

#[derive(Queryable)]
pub struct User {
  pub id: String,
  pub username: String,
  pub password: String,
  pub avatar: String,
  pub created_at: DateTime<Local>
}
