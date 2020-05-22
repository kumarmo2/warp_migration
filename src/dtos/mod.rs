use cookie::Cookie;
use serde::Serialize;
use serde_json::to_string;
use warp::hyper::body::Body;
use warp::{http::StatusCode, reply::Reply};

pub mod users {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct CreateUserRequest {
        pub name: String,
        pub age: Option<i32>,
        pub email: String,
        pub password: String,
    }
}

#[derive(Serialize)]
pub struct CustomResponse<R, E>
where
    R: Serialize + Send,
    E: Serialize + Send,
{
    // Its the responsibility of the user, to make sure that either result or error should be set.
    result: Option<R>,
    error: Option<E>,
    #[serde(skip_serializing)]
    status: StatusCode,
}

impl<R, E> Default for CustomResponse<R, E>
where
    R: Serialize + Send,
    E: Serialize + Send,
{
    fn default() -> Self {
        Self {
            result: None,
            error: None,
            status: StatusCode::OK,
        }
    }
}
impl<R, E> CustomResponse<R, E>
where
    R: Serialize + Send,
    E: Serialize + Send,
{
    pub fn with_result(result: R) -> Self {
        Self {
            result: Some(result),
            ..CustomResponse::default()
        }
    }
    pub fn with_error(error: E, status: StatusCode) -> Self {
        Self {
            error: Some(error),
            status,
            ..CustomResponse::default()
        }
    }
}

impl<R, E> Reply for CustomResponse<R, E>
where
    R: Serialize + Send,
    E: Serialize + Send,
{
    fn into_response(self) -> warp::http::response::Response<warp::hyper::body::Body> {
        let builder = warp::http::response::Response::builder().status(self.status);

        let body = to_string(&self).unwrap();
        let body = Body::from(body);
        return builder.body(body).unwrap();
    }
}
