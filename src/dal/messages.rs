use crate::models::messages::Message;
use sqlx::{cursor::Cursor, row::Row, MySqlPool};

pub async fn get_messages_of_room(id: i32, conn: &MySqlPool) -> Result<Vec<Message>, sqlx::Error> {
    let mut cursor = sqlx::query("select * from messages where room_id = ?")
        .bind(id)
        .fetch(conn);
    let mut messages = Vec::new();

    while let Some(row) = cursor.next().await? {
        let id = row.try_get("id")?;
        let room_id = row.try_get("room_id")?;
        let sender_id = row.try_get("sender_id")?;
        let content = row.try_get("content")?;
        let message = Message {
            id,
            room_id,
            sender_id,
            content,
        };
        messages.push(message);
    }
    Ok(messages)
}
