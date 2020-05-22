use crate::handlers::users::create;
use sqlx::MySqlPool;
use warp::Filter;

pub fn get_all_filter(
    pool: MySqlPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let api_v1 = warp::path!("api" / "v1" / ..);

    let users_api_prefix = warp::path("users");

    let user_create_filter = warp::body::json()
        .and(with_db(pool.clone()))
        .and_then(create);
    // .map();

    let users = users_api_prefix.and(user_create_filter);

    let post_routes = warp::post().and(api_v1.and(users));

    let routes = post_routes;
    return routes;
}

pub fn with_db(
    conn: MySqlPool,
    // TODO: read about Infallible
) -> impl Filter<Extract = (MySqlPool,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || conn.clone())
}
