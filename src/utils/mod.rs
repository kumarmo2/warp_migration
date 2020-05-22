pub mod users {
    use cookie::{Cookie, CookieBuilder};
    use jsonwebtoken::{Algorithm, Header, encode};

    use crate::dtos::users::UserJwtPayload;

    pub generate_user_cookie(user_id: i32) -> Cookie {

    }
}