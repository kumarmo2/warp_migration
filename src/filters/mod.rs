pub mod common;
pub mod users;

use sqlx::MySqlPool;
use warp::Filter;

pub fn get_all_filter(
    pool: MySqlPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let api_v1 = warp::path!("api" / "v1" / ..);

    let users_endpoints = users::get_filters(pool.clone());
    let routes = api_v1.and(users_endpoints);

    return routes;
}
