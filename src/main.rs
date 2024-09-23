use sqlx::{query, Pool, Postgres};
use dotenv::dotenv;
use std::env;
use sqlx::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Load environment variables from .env
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    add_users(&database_url).await?;
    read_users(&database_url).await?;
    Ok(())
}
async fn alter_user(database_url:&str)-> Result<(), Error>{
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

    Ok(())
}
async fn alter_books(database_url:&str)-> Result<(), Error>{
    let pool = Pool::<Postgres>::connect(&database_url).await?;
    //batch insert
    let mut query = sqlx::query(
        "INSERT INTO books(title, author) 
         VALUES ($1, $2), ($3, $4), ($5, $6)"
    )
    .bind("The Catcher in the Rye")
    .bind("J.D. Salinger")
    .bind("1984")
    .bind("George Orwell")
    .bind("Brave New World")
    ;
    
    query = query.bind("Aldous Huxley");
    query.execute(&pool).await?;

    Ok(())
}
async fn add_users(database_url:&str)-> Result<(), Error>{
    let pool = Pool::<Postgres>::connect(&database_url).await?;
    let mut q = String::from("INSERT INTO users (name) VALUES");
    let v = vec!["harry potter", "J K Rowling"];
    for i in (1..3){
        q = q+"($"+ &i.to_string()+")";
        if i!=2 {q=q+","}
    }
    let mut query = sqlx::query(&q);
    for x in v{
        query = query.bind(x);
    }
    println!("{}",q);
    query.execute(&pool).await?;
    println!("Added books");
    Ok(())
}
async fn read_users(database_url:&str)-> Result<(), Error>{
    let pool = Pool::<Postgres>::connect(&database_url).await?;
    // Fetch the inserted row
    let rows = sqlx::query!("SELECT id, name FROM users")
        .fetch_all(&pool)
        .await?;

    for row in rows {
        println!("User {}: {}", row.id, row.name);
    }
    
    Ok(())
}
async fn read_books(database_url:&str)-> Result<(), Error>{
    let pool = Pool::<Postgres>::connect(&database_url).await?;
    // Fetch the inserted row
    let rows = sqlx::query!("SELECT id, title, author FROM books")
        .fetch_all(&pool)
        .await?;

    for row in rows {
        println!("Book {}: {}, {}", row.id, row.title, row.author);
    }

    Ok(())
}