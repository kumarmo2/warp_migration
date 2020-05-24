pub mod users {
    use crate::constants::JWT_SECRET;
    use cookie::{Cookie, CookieBuilder};
    use jsonwebtoken::{encode, Algorithm, Header};
    use time::Duration;

    use crate::dtos::users::UserJwtPayload;

    pub fn generate_user_cookie(user_id: i32) -> Cookie<'static> {
        let payload = UserJwtPayload {
            id: user_id,
            exp: 10000000000,
        };
        let header = Header::new(Algorithm::HS256);
        let token = encode(&header, &payload, JWT_SECRET).unwrap();
        CookieBuilder::new("auth", token)
            .http_only(true)
            .path("/")
            .max_age(Duration::days(365))
            .finish()
    }
}
