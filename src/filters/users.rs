use crate::{
    business::users::create as create_bl, dtos::users::CreateUserRequest, filters::common::with_db,
};
use sqlx::MySqlPool;
use validator::validate_email;
use warp::http::{response::Response, StatusCode};
use warp::hyper::body::Body;
use warp::{reject::Reject, Filter};

use crate::{
    dtos::{response::Error, users::User as UserDto},
    models::users::UserResult,
};

pub fn get_filters(
    pool: MySqlPool,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let users_api_prefix = warp::path("users");

    /*
        POST: /users
    */
    let user_create_filter = warp::body::json()
        .and_then(|request: CreateUserRequest| async move {
            match validate_create_user_request(&request) {
                // TODO: send appropriate error
                Some(reason) => Err(warp::reject::reject()),
                None => Ok(request),
            }
        })
        .and(with_db(pool.clone()))
        .and_then(create);

    let post_end_points = warp::post().and(user_create_filter);

    users_api_prefix.and(post_end_points)
}

async fn create(
    user_request: CreateUserRequest,
    conn: MySqlPool,
) -> Result<impl warp::reply::Reply, std::convert::Infallible> {
    let user;
    match create_bl(&user_request, &conn).await {
        Ok(result) => match result {
            UserResult::EmailAlreadyExists => {
                return Ok(Response::builder().status(StatusCode::OK).body(Body::from(
                    serde_json::to_string(&Error::new("Email already in use")).unwrap(),
                )));
            }
            UserResult::Model(u) => {
                user = u;
            }
        },
        Err(reason) => {
            println!("could not create user: {}", reason);
            return Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::empty()));
        }
    }
    // TODO: Set cookie as well.
    // let x = with_header(response, "sdfs", "dfss");
    let x = Ok(Response::builder()
        // .header("sdfsdl", "dsfsdf")
        .body(Body::from(
            serde_json::to_string(&UserDto::from(user)).unwrap(),
        )));
    x
}

pub fn validate_create_user_request(user_request: &CreateUserRequest) -> Option<&'static str> {
    if !validate_email(&user_request.email) {
        return Some("Invalid email");
    }

    if let Some(age_input) = user_request.age {
        if age_input < 18 || age_input > 100 {
            return Some("age must be between 18 and 100");
        }
    }
    if user_request.password.len() < 6 || user_request.password.len() > 20 {
        return Some("Password must be between 6 and 20 characters");
    }
    None
}
