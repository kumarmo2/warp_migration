use sqlx::cursor::Cursor;
use sqlx::mysql::MySqlPool;
use sqlx::mysql::MySqlQueryAs;
use warp::Filter;

mod dtos;
mod handlers;
mod models;

use handlers::users::create;
use models::User;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = MySqlPool::builder()
        .max_size(5)
        .build("mysql://root:root@127.0.0.1/practice")
        .await?;

    let stream = sqlx::query_as::<_, User>("select * from users").fetch(&pool);

    Ok(())
    // println!("Hello, world!");
    // let hello = warp::path!("hello" / String).map(|name| format!("hello, {}", name));

    // let api_v1 = warp::path!("api" / "v1" / ..);

    // let users = warp::path("users").and(warp::body::json()).map(create);

    // let post_routes = warp::post().and(api_v1.and(users));

    // let get_routes = warp::get().and(api_v1.and(hello));

    // let routes = post_routes.or(get_routes);
    // warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
}
