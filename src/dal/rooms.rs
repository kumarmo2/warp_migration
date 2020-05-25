use crate::models::rooms::Room;
use sqlx::MySqlPool;

// pub async fn get_room_by_path(path: &str, conn: &MySqlPool) -> Result<Room, sqlx::Error> {
//     sqlx::query_as!(Room, "select * from rooms where url_identifier = ?", path)
//         .fetch_one(conn)
//         .await
// }
