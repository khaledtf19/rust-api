use axum::{
    routing::get,
    Router,
};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Executor, Row};

#[derive(sqlx::FromRow, Debug)]
struct CC {
    personid: i32
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect("postgres://root:example@localhost:8000").await.unwrap();

    let q = sqlx::query_as::<_,CC>("SELECT * FROM cc").fetch_all(&pool).await?;

    // // let row= sqlx::query_as("create table CC (PersonID int)").fetch_one(&pool).await?;

    println!("Got: {:?}", q);
    
    println!("Hello, world!");
    let app = Router::new().route("/", get(|| async  {"Hello 123213213"}) );
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener,app).await.unwrap();


    Ok(())
}
