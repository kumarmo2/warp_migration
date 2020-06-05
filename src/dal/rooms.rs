use crate::models::rooms::{Room, RoomSubscriber};
use sqlx::{prelude::*, row::Row, MySqlPool};

pub async fn get_room_by_path(path: &str, conn: &MySqlPool) -> Result<Room, sqlx::Error> {
    sqlx::query_as::<_, Room>("select * from rooms where url_identifier = ? limit 1")
        .bind(path)
        .fetch_one(conn)
        .await
}

pub async fn get_room_by_id(id: i32, conn: &MySqlPool) -> Result<Room, sqlx::Error> {
    sqlx::query_as::<_, Room>("select * from rooms where id = ? limit 1")
        .bind(id)
        .fetch_one(conn)
        .await
}

pub async fn get_room_subscribers(
    id: i32,
    pool: &MySqlPool,
) -> Result<Vec<RoomSubscriber>, sqlx::Error> {
    // TODO: Check if there is another type safe approach to achieve this.
    let mut cursor = sqlx::query("select * from roomsubscribers where room_id = ?")
        .bind(id)
        .fetch(pool);

    let mut subs = Vec::new();
    while let Some(row) = cursor.next().await? {
        let room_id: i32 = row.try_get("room_id")?;
        let member_id: i32 = row.try_get("member_id")?;
        let sub = RoomSubscriber { member_id, room_id };
        subs.push(sub);
    }
    Ok(subs)
}
