use diesel_async::{AsyncPgConnection, AsyncConnection};
use dotenvy::dotenv;

pub async fn establish_connection() -> AsyncPgConnection {
  dotenv().ok();

  let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  AsyncPgConnection::establish(&database_url).await.unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
