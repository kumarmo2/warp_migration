mod common;
mod rooms;
mod users;

use crate::filters::common::handle_rejection;
use sqlx::MySqlPool;
use warp::Filter;

pub fn get_all_filter(
    pool: MySqlPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let api_v1 = warp::path!("api" / "v1" / ..);
    let logger = warp::log::log("RequestLogger");

    let users_endpoints = users::get_filters(pool.clone());
    let rooms_endpoints = rooms::get_all_filters(pool.clone());
    let routes = api_v1.and(
        users_endpoints
            .or(rooms_endpoints)
            .recover(handle_rejection),
    );
    let routes = routes.with(logger);

    return routes;
}
