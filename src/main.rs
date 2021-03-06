use sqlx::mysql::MySqlPool;

mod business;
mod constants;
mod dal;
mod dtos;
mod filters;
mod handlers;
mod models;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: check if we can do it lazily
    // Run mysql docker container before starting.
    let pool = MySqlPool::builder()
        .max_size(5)
        // TODO: need to think about how to keep this dynamic
        .build("mysql://root:admin@localhost/practice")
        .await?;

    // TODO: make it sit behind nginx.
    println!("Hello, world!");
    let routes = filters::get_all_filter(pool);

    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
    Ok(())
}
