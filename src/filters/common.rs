use crate::{
    constants::USER_COOKIE_NAME,
    dal::users::get_by_id,
    dtos::{response::Error, users::UserJwtPayload},
    models::users::User,
    utils::users::get_payload_from_user_cookie_str,
};
use core::convert::Infallible;
use sqlx::MySqlPool;
use warp::{filters::cookie::cookie, http::StatusCode, Filter, Rejection, Reply};

pub fn with_db(
    conn: MySqlPool,
    // TODO: read about Infallible
) -> impl Filter<Extract = (MySqlPool,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || conn.clone())
}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code: u16;
    let error_message;
    if err.is_not_found() {
        code = 404;
        error_message = "NOT_FOUND";
    } else if let Some(e) = err.find::<Error>() {
        code = e.code;
        error_message = e.error;
    } else {
        code = 500;
        error_message = "Internal Server Error!";
    }

    println!("from handle_rejection, message: {:?}", err);
    let error_response = warp::reply::json(&Error::new(error_message, code));
    Ok(warp::reply::with_status(
        error_response,
        StatusCode::from_u16(code).unwrap(),
    ))
}

pub fn with_user(conn: MySqlPool) -> impl Filter<Extract = (User,), Error = Rejection> + Clone {
    cookie(USER_COOKIE_NAME)
        .and_then(|user_cookie: String| async move {
            match get_payload_from_user_cookie_str(&user_cookie) {
                Ok(token_data) => Ok(token_data.claims),
                Err(_) => Err(warp::reject::custom(Error::new("Unauthenticated", 401))),
            }
        })
        .and(with_db(conn))
        .and_then(|jwt: UserJwtPayload, conn: MySqlPool| async move {
            if jwt.id < 1 {
                return Err(warp::reject::custom(Error::new("Invalid user", 401)));
            }
            match get_by_id(jwt.id, &conn).await {
                Ok(u) => Ok(u),
                Err(_) => Err(warp::reject::custom(Error::new("Invalid user", 401))),
            }
        })
}
