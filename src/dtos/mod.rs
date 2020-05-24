pub mod users {
    use crate::models::users::User as UserModel;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct CreateUserRequest {
        pub name: String,
        pub age: Option<i32>,
        pub email: String,
        pub password: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct User {
        id: i32,
        name: String,
        age: Option<i32>,
        email: String,
    }

    impl From<UserModel> for User {
        fn from(model: UserModel) -> Self {
            Self {
                id: model.id,
                name: model.name,
                age: model.age,
                email: model.email,
            }
        }
    }

    impl User {
        pub fn get_id(&self) -> i32 {
            self.id
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct UserJwtPayload {
        pub id: i32,
        pub exp: u64,
    }
}

pub mod response {
    use serde::Serialize;
    use warp::reject::Reject;

    #[derive(Serialize, Debug)]
    pub struct Error {
        pub error: &'static str,
        #[serde(skip_serializing)]
        pub code: u16,
    }

    impl Reject for Error {}

    impl Error {
        pub fn new(error: &'static str, code: u16) -> Self {
            Self { error, code }
        }
    }
}
