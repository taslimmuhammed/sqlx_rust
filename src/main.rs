use std::error::Error;
use sqlx::postgres::PgPool;
use sqlx::{Connection, Executor, Row};

#[tokio::main]
async fn main() -> Result<(),Box<dyn Error>>{
    Ok(())
}
async fn connect_using_pool()->Result<i32,Box<dyn Error>>{
    let email: &str="";
    let pool = PgPool::connect("postgres://taslim:6318@localhost:5432/test").await?;
    let mut rows = sqlx::query("SELECT * FROM users WHERE email = ?")
    .bind(email)
    .fetch(&pool);
    Ok(1)
}
async fn connect_db()->Result<i32,Box<dyn Error>>{
    let url = "postgres://taslim:6318@localhost:5432/test";
    let mut conn = sqlx::postgres::PgConnection::connect(url).await?;
    let res = sqlx::query("SELECT 1+1 as sum").fetch_one(&mut conn).await?;
    let sum: i32 = res.get("sum");
    Ok(sum)
}