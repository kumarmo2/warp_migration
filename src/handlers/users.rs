use sqlx::MySqlPool;
use validator::validate_email;
use warp::http::{response::Response, StatusCode};
use warp::hyper::body::Body;

use crate::{
    business::users::create as create_bl,
    dtos::{
        response::Error,
        users::{CreateUserRequest, User as UserDto},
    },
    models::users::UserResult,
};

pub async fn create(
    user_request: CreateUserRequest,
    conn: MySqlPool,
) -> Result<impl warp::reply::Reply, std::convert::Infallible> {
    if let Some(reason) = validate_create_user_request(&user_request) {
        let err = Error::new(String::from(reason));
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(serde_json::to_string(&err).unwrap())));
    }
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
        .header("sdfsdl", "dsfsdf")
        .body(Body::from(
            serde_json::to_string(&UserDto::from(user)).unwrap(),
        )));
    x
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
