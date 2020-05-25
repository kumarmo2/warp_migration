use super::common::{with_db, with_user};
use crate::{dtos::rooms::RoomDetails, models::users::User};
use sqlx::MySqlPool;
use warp::{path, Filter, Rejection, Reply};

pub fn get_all_filters(
    conn: MySqlPool,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let api_prefix = path("rooms");

    let get_by_path = api_prefix
        .and(warp::path::param())
        .and(with_user(conn.clone()))
        .and(with_db(conn.clone()))
        .map(|room_path: String, user, db| "kumarmo2");

    warp::get().and(get_by_path)
}

// async fn get_room_details_by_path(
//     path: String,
//     _: User, //This parameter was just for user authentication.
//     pool: MySqlPool,
// ) -> Result<RoomDetails, Rejection> {

// }
