use super::common::{with_db, with_user};
use sqlx::MySqlPool;
use warp::{path, Filter, Rejection, Reply};

pub fn get_all_filters(
    conn: MySqlPool,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let api_prefix = path("rooms");

    let get_by_path = api_prefix
        .and(with_user(conn.clone()))
        .and(with_db(conn.clone()))
        .map(|user, db| "kumarmo2");

    warp::get().and(get_by_path)
}
