#[derive(sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub age: Option<i32>,
    pub email: String,
    pub password: String,
}
