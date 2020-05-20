use crate::dtos::users::CreateUserRequest;
use validator::validate_email;
use warp::http::StatusCode;
use warp::reply::{with_status, WithStatus};

pub fn create(user_request: CreateUserRequest) -> WithStatus<impl warp::reply::Reply> {
    if let Some(status) = validate_create_user_request(&user_request) {
        return status;
    }
    with_status("Created", StatusCode::from_u16(200).unwrap())
}

fn validate_create_user_request(
    user_request: &CreateUserRequest,
) -> Option<WithStatus<&'static str>> {
    if !validate_email(&user_request.email) {
        return Some(with_status(
            "Invalid email",
            StatusCode::from_u16(400).unwrap(),
        ));
    }

    if let Some(age_input) = user_request.age {
        if age_input < 18 || age_input > 100 {
            return Some(with_status(
                "age must be between 18 and 100",
                StatusCode::from_u16(400).unwrap(),
            ));
        }
    }
    if user_request.password.len() < 6 || user_request.password.len() > 20 {
        return Some(with_status(
            "Password must be between 6 and 20 characters",
            StatusCode::from_u16(400).unwrap(),
        ));
    }
    None
}
