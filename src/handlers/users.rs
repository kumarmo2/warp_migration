use sqlx::MySqlPool;
use validator::validate_email;
use warp::http::StatusCode;

use crate::{
    business::users::create as create_bl,
    dtos::{users::CreateUserRequest, CustomResponse},
    models::users::UserResult,
};

pub async fn create(
    user_request: CreateUserRequest,
    conn: MySqlPool,
) -> Result<impl warp::reply::Reply, std::convert::Infallible> {
    if let Some(reason) = validate_create_user_request(&user_request) {
        return Ok(CustomResponse::with_error(reason, StatusCode::BAD_REQUEST));
    }
    let user;
    match create_bl(&user_request, &conn).await {
        Ok(result) => match result {
            UserResult::EmailAlreadyExists => {
                return Ok(CustomResponse::with_error(
                    "email already exists",
                    StatusCode::OK,
                ));
            }
            UserResult::Model(u) => {
                user = u;
            }
        },
        Err(reason) => {
            println!("could not create user: {}", reason);
            return Ok(CustomResponse::with_error(
                "Internal Server Error",
                StatusCode::INTERNAL_SERVER_ERROR,
            ));
        }
    }
    // TODO: Set cookie as well.
    Ok(CustomResponse::with_result(()))
}

fn validate_create_user_request(user_request: &CreateUserRequest) -> Option<&'static str> {
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
