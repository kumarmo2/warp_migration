use super::common::{with_db, with_user};
use crate::{
    dal::{
        messages::get_messages_of_room,
        rooms::{
            get_room_by_id as get_room_by_id_dal, get_room_by_path as get_room_by_path_dal,
            get_room_subscribers as get_room_subscribers_dal,
        },
        users::get_by_id,
    },
    dtos::{messages::Message, response::Error, rooms::RoomDetails, users::UserDto},
    models::{
        rooms::{Room, RoomSubscriber},
        users::User,
    },
};
use sqlx::MySqlPool;
use std::convert::Infallible;
use warp::{path, Filter, Rejection, Reply};

pub fn get_all_filters(
    conn: MySqlPool,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let api_prefix = path("rooms");
    let get_by_path = api_prefix
        .and(warp::path::param())
        .and(warp::path::end())
        .and(with_user(conn.clone()))
        .and(with_db(conn.clone()))
        .and_then(get_room_by_path)
        .and_then(get_room_subscribers)
        .and_then(get_room_details);

    let room_messages = with_user(conn.clone())
        .and(api_prefix)
        .and(warp::path::param())
        .and(warp::path::path("messages"))
        .and(warp::path::end())
        .and(with_db(conn.clone()))
        .and_then(get_room_by_id)
        .and_then(get_room_messages);

    warp::get().and(get_by_path.or(room_messages))
}

async fn get_room_by_id(
    _user: User,
    id: i32,
    conn: MySqlPool,
) -> Result<(Room, MySqlPool), Rejection> {
    match get_room_by_id_dal(id, &conn).await {
        Ok(room) => Ok((room, conn)),
        Err(reason) => {
            println!("error: {}", reason);
            Err(warp::reject::custom(Error::new("Could not find room", 404)))
        }
    }
}

async fn get_room_messages((room, conn): (Room, MySqlPool)) -> Result<impl Reply, Rejection> {
    match get_messages_of_room(room.id, &conn).await {
        Ok(messages) => {
            let result: Vec<Message> = messages.into_iter().map(|m| Message::from(m)).collect();
            Ok(warp::reply::json(&result))
        }
        Err(reason) => {
            println!("error, reason: {}", reason);
            Err(warp::reject::custom(Error::new(
                "Something went wrong",
                500,
            )))
        }
    }
}

async fn get_room_details(
    (subs, room, pool): (Vec<RoomSubscriber>, Room, MySqlPool),
) -> Result<impl Reply, Infallible> {
    let mut users: Vec<User> = Vec::new();

    for sub in subs {
        match get_by_id(sub.member_id, &pool).await {
            Ok(user) => users.push(user),
            Err(reason) => {
                println!("error: {}", reason);
            }
        }
    }

    let users_dto: Vec<UserDto> = users
        .into_iter()
        .map(|model| UserDto::from(model))
        .collect();

    let mut room_details = RoomDetails::new(room.id, room.name, room.creator_user_id);

    for user in users_dto.into_iter() {
        room_details.add_member(user);
    }

    Ok(warp::reply::json(&room_details))
}

async fn get_room_subscribers(
    (room, pool): (Room, MySqlPool),
) -> Result<(Vec<RoomSubscriber>, Room, MySqlPool), Rejection> {
    match get_room_subscribers_dal(room.id, &pool).await {
        Ok(subs) => Ok((subs, room, pool)),
        Err(reason) => {
            println!("error: {}", reason);
            return Err(warp::reject::custom(Error::new(
                "Internal Server Error",
                500,
            )));
        }
    }
}

async fn get_room_by_path(
    path: String,
    _: User, //This parameter was just for user authentication.
    pool: MySqlPool,
) -> Result<(Room, MySqlPool), Rejection> {
    match get_room_by_path_dal(&path, &pool).await {
        Ok(room) => Ok((room, pool)),
        Err(reason) => {
            println!("error occured, reason: {:?}", reason);
            Err(warp::reject::custom(Error::new("Room Not Found", 404)))
        }
    }
}
