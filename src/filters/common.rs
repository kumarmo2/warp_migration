use crate::dtos::response::Error;
use core::convert::Infallible;
use sqlx::MySqlPool;
use warp::{http::StatusCode, Filter, Rejection, Reply};

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

    println!("from handle_rejection, message: {}", error_message);
    let error_response = warp::reply::json(&Error::new(error_message, code));
    Ok(warp::reply::with_status(
        error_response,
        StatusCode::from_u16(code).unwrap(),
    ))
}
