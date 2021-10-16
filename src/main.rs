use tide::Request;
use tide::prelude::*;
use sqlx::postgres::PgPoolOptions;

#[derive(Debug, Deserialize)]
struct Animal {
    name: String,
    legs: u8,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    // Postgres connection
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://username:password@127.0.0.1:5432/database").await?;

    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool).await?;

    println!("row {}", row.0);
    assert_eq!(row.0, 150);

    let mut app = tide::new();
    app.at("/orders/shoes").post(order_shoes);
    app.listen("127.0.0.1:8080").await?;
    println!("Tide Server running at 127.0.0.1:8080");
    Ok(())
}

async fn order_shoes(mut req: Request<()>) -> tide::Result {
    let Animal { name, legs } = req.body_json().await?;
    Ok(format!("Hello, {}! I've put in an order for {} shoes", name, legs).into())
}
