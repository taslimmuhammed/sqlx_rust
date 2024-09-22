use sqlx::{Pool, Postgres};
use dotenv::dotenv;
use std::env;
use sqlx::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Load environment variables from .env
    dotenv().ok();

    // Get the database URL from the environment
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Create a connection pool to the database
    let pool = Pool::<Postgres>::connect(&database_url).await?;

    // Create a new table (if it doesn't already exist)
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name VARCHAR(100) NOT NULL
        );"
    )
    .execute(&pool)
    .await?;
    //batch insert
    sqlx::query!(
        "INSERT INTO users (name) VALUES ($1), ($2), ($3)",
        "Alice", "Bob", "Charlie"
    ).execute(&pool).await?;

    // Insert a row into the table
    sqlx::query("INSERT INTO users (name) VALUES ($1)")
        .bind("Alice")
        .execute(&pool)
        .await?;

    // Fetch the inserted row
    let rows = sqlx::query!("SELECT id, name FROM users")
        .fetch_all(&pool)
        .await?;

    for row in rows {
        println!("User {}: {}", row.id, row.name);
    }
    
    Ok(())
}

// async fn connect_db()->Result<i32,Error>{
//     let url = "postgres://taslim:6318@localhost:5432/test";
//     let mut conn = sqlx::postgres::PgConnection::connect(url).await?;
//     let res = sqlx::query("SELECT 1+1 as sum").fetch_one(&mut conn).await?;
//     let sum: i32 = res.get("sum");
//     Ok(sum)
// }