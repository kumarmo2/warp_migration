use crate::dtos::users::CreateUserRequest;
use crate::models::users::User;
use sqlx::MySqlPool;

pub async fn get_by_email(email: &str, conn: &MySqlPool) -> Result<User, sqlx::Error> {
    let result = sqlx::query_as!(User, "select * from users where email = ?", email)
        .fetch_one(conn)
        .await?;

    Ok(result)
}

pub async fn create(
    user: &CreateUserRequest,
    hash_pass: &str,
    conn: &MySqlPool,
) -> Result<User, sqlx::error::Error> {
    let _ = sqlx::query!(
        "insert into users (name, age, email, password) values (?, ?, ?, ?)",
        user.name,
        user.age,
        user.email,
        hash_pass
    )
    .execute(conn)
    .await?;

    // TODO: Make this is an single call and select only id from db.
    let user = sqlx::query_as!(User, "select * from users where email = ?", user.email)
        .fetch_one(conn)
        .await?;

    Ok(user)
}
