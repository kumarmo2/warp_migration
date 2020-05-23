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

    #[derive(Debug, Serialize, Deserialize)]
    pub struct UserJwtPayload {
        id: i32,
    }
}

pub mod response {
    use serde::Serialize;

    #[derive(Serialize)]
    pub struct Error<E> {
        error: E,
    }

    impl<E> Error<E>
    where
        E: Serialize,
    {
        pub fn new(error: E) -> Self {
            Error { error }
        }
    }
}
