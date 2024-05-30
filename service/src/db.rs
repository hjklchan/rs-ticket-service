use sqlx::{mysql::MySqlPoolOptions, MySqlPool};

pub async fn connect() -> MySqlPool {
    MySqlPoolOptions::new().connect("mysql://root:mypass@localhost:3306/ticket_service").await.unwrap()
}
