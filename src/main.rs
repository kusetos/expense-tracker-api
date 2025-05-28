use actix_web::{App, HttpServer};

pub mod routes;
mod handlers;
mod models;
mod utils;
mod middleware;


#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    let url = "postgres://postgres:secret@localhost:5432/postgres";
    //let pool = sqlx::postgres::PgPool::connect(url).await.unwrap();


    HttpServer::new(move || {
        App::new()
            //.app_data(actix_web::web::Data::new(pool.clone()))
            .configure(routes::auth::config)
            .configure(routes::expenses::config)

    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await

}
