use crate::{
    business::users::create as create_bl, dtos::users::CreateUserRequest, filters::common::with_db,
    utils::users::generate_user_cookie,
};
use serde_json::to_string;
use sqlx::MySqlPool;
use std::convert::Infallible;
use validator::validate_email;
use warp::{http::response::Response, hyper::body::Body, Filter, Rejection, Reply};

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
        .and_then(validate_create_user_request)
        .and(with_db(pool.clone()))
        .and_then(create)
        .and_then(create_reply);

    let post_end_points = warp::post().and(user_create_filter);

    users_api_prefix.and(post_end_points)
}

async fn create_reply(user: UserDto) -> Result<impl Reply, Infallible> {
    let cookie = generate_user_cookie(user.get_id());
    Ok(Response::builder()
        .header("Set-Cookie", cookie.to_string())
        .body(Body::from(to_string(&user).unwrap())))
}

async fn create(
    user_request: CreateUserRequest,
    conn: MySqlPool,
) -> Result<UserDto, warp::Rejection> {
    let user;
    match create_bl(&user_request, &conn).await {
        Ok(result) => match result {
            UserResult::EmailAlreadyExists => {
                return Err(warp::reject::custom(Error::new(
                    "Email Already Exists",
                    200,
                )));
            }
            UserResult::Model(u) => {
                user = u;
            }
        },
        Err(reason) => {
            println!("could not create user: {}", reason);
            return Err(warp::reject::custom(Error::new(
                "Internal Server Error",
                500,
            )));
        }
    }
    Ok(UserDto::from(user))
}

async fn validate_create_user_request<'a>(
    user_request: CreateUserRequest,
) -> Result<CreateUserRequest, Rejection> {
    if !validate_email(&user_request.email) {
        return Err(warp::reject::custom(Error::new("Invalid email", 400)));
    }

    if let Some(age_input) = user_request.age {
        if age_input < 18 || age_input > 100 {
            return Err(warp::reject::custom(Error::new(
                "age must be between 18 and 100",
                400,
            )));
        }
    }
    if user_request.password.len() < 6 || user_request.password.len() > 20 {
        return Err(warp::reject::custom(Error::new(
            "Password must be between 6 and 20 characters",
            400,
        )));
    }
    Ok(user_request)
}
