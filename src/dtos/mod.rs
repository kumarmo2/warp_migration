pub mod users {
    use crate::models::users::User;
    use mapper::Mapper;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct CreateUserRequest {
        pub name: String,
        pub age: Option<i32>,
        pub email: String,
        pub password: String,
    }

    #[derive(Debug, Serialize, Deserialize, Mapper)]
    #[from(User)]
    pub struct UserDto {
        id: i32,
        name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        age: Option<i32>,
        email: String,
    }

    impl UserDto {
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

pub mod messages {
    use crate::models::messages::Message as MessageModel;
    use mapper::Mapper;
    use serde::{Deserialize, Serialize};

    #[derive(Mapper, Serialize, Deserialize)]
    #[from(MessageModel)]
    pub struct Message {
        pub id: i32,
        pub room_id: i32,
        pub sender_id: i32,
        pub content: String,
    }
}

pub mod rooms {
    use super::users::UserDto;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct RoomDetails {
        id: i32,
        name: String,
        #[serde(rename = "creatorUserId")]
        creator_user_id: i32,
        members: Vec<UserDto>,
    }

    impl RoomDetails {
        pub fn new(id: i32, name: String, creator_user_id: i32) -> Self {
            Self {
                id,
                name,
                creator_user_id,
                members: Vec::new(),
            }
        }
    }

    impl RoomDetails {
        pub fn add_member(&mut self, member: UserDto) {
            self.members.push(member);
        }
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
