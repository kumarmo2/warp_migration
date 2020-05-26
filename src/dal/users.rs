use crate::dtos::users::CreateUserRequest;
use crate::models::users::User;
use sqlx::{mysql::MySqlQueryAs, MySqlPool};

pub async fn get_by_email(email: &str, conn: &MySqlPool) -> Result<User, sqlx::Error> {
    let result = sqlx::query_as::<_, User>("select * from users where email = ?")
        .bind(email)
        .fetch_one(conn)
        .await?;

    Ok(result)
}

pub async fn get_by_id(id: i32, conn: &MySqlPool) -> Result<User, sqlx::Error> {
    sqlx::query_as::<_, User>("select * from users where id = ?")
        .bind(id)
        .fetch_one(conn)
        .await
}

pub async fn create(
    user: &CreateUserRequest,
    hash_pass: &str,
    conn: &MySqlPool,
) -> Result<User, sqlx::error::Error> {
    let _ = sqlx::query("insert into users (name, age, email, password) values (?, ?, ?, ?)")
        .bind(&user.name)
        .bind(user.age)
        .bind(&user.email)
        .bind(hash_pass)
        .execute(conn)
        .await?;

    // TODO: Make this is an single call and select only id from db.
    let user = sqlx::query_as::<_, User>("select * from users where email = ?")
        .bind(&user.email)
        .fetch_one(conn)
        .await?;

    Ok(user)
}
