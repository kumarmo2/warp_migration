use crate::{
    dal::users::{create as create_dal, get_by_email},
    dtos::users::CreateUserRequest,
    models::users::UserResult,
};
use bcrypt::hash;
use sqlx::{Error, MySqlPool};

pub async fn create(
    request: &CreateUserRequest,
    conn: &MySqlPool,
) -> Result<UserResult, sqlx::Error> {
    match get_by_email(&request.email, conn).await {
        Ok(_) => {
            println!("user already exists");
            return Ok(UserResult::EmailAlreadyExists);
        }
        Err(reason) => match reason {
            Error::RowNotFound => {
                println!("reason: no row found");
            }
            _ => {
                println!("here: {}", reason);
                return Err(reason);
            }
        },
    }
    let pass = String::from(&request.password);
    let pass = hash(pass, bcrypt::DEFAULT_COST).unwrap();

    let user = create_dal(request, &pass, conn).await?;
    Ok(UserResult::Model(user))
}
