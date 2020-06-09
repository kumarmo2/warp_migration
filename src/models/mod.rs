pub mod users {
    use serde::Serialize;
    #[derive(sqlx::FromRow, Debug, Serialize)]
    pub struct User {
        pub id: i32,
        pub name: String,
        pub age: Option<i32>,
        pub email: String,
        pub password: String,
    }
    pub enum UserResult {
        EmailAlreadyExists,
        Model(User),
    }
}

pub mod messages {
    #[derive(sqlx::FromRow, Debug)]
    pub struct Message {
        pub id: i32,
        pub room_id: i32,
        pub sender_id: i32,
        pub content: String,
    }
}

pub mod rooms {
    #[derive(sqlx::FromRow, Debug)]
    pub struct Room {
        pub id: i32,
        pub name: String,
        pub creator_user_id: i32,
        pub url_identifier: String,
        pub is_public: bool,
    }

    #[derive(sqlx::FromRow, Debug)]
    pub struct RoomSubscriber {
        pub member_id: i32,
        pub room_id: i32,
    }
}
