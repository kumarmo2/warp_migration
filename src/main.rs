use sqlx::mysql::MySqlPool;
use warp::Filter;

mod business;
mod dal;
mod dtos;
mod filters;
mod handlers;
mod models;

use filters::with_db;
use handlers::users::create;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: check if we can do it lazily
    // warp::log::custom(func: F)
    let pool = MySqlPool::builder()
        .max_size(5)
        .build("mysql://root:root@127.0.0.1/practice")
        .await?;

    // TODO: make it sit behind nginx.
    println!("Hello, world!");
    let hello = warp::path!("hello" / String).map(|name| format!("hello, {}", name));

    let api_v1 = warp::path!("api" / "v1" / ..);

    let users = warp::path("users")
        .and(warp::body::json())
        .and(with_db(pool.clone()))
        .and_then(create);

    let post_routes = warp::post().and(api_v1.and(users));

    let get_routes = warp::get().and(api_v1.and(hello));

    let routes = post_routes.or(get_routes);
    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
    Ok(())
}
