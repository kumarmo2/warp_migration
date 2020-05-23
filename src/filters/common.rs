use sqlx::MySqlPool;
use warp::Filter;

pub fn with_db(
    conn: MySqlPool,
    // TODO: read about Infallible
) -> impl Filter<Extract = (MySqlPool,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || conn.clone())
}
