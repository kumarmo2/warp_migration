pub mod users {
    use crate::constants::{JWT_SECRET, USER_COOKIE_NAME};
    use cookie::{Cookie, CookieBuilder};
    use jsonwebtoken::{decode, encode, Algorithm, Header, TokenData, Validation};
    use time::Duration;

    use crate::dtos::users::UserJwtPayload;

    pub fn generate_user_cookie(user_id: i32) -> Cookie<'static> {
        let payload = UserJwtPayload {
            id: user_id,
            exp: 10000000000,
        };
        let header = Header::new(Algorithm::HS256);
        let token = encode(&header, &payload, JWT_SECRET).unwrap();
        CookieBuilder::new(USER_COOKIE_NAME, token)
            .http_only(true)
            .path("/")
            .max_age(Duration::days(365))
            .finish()
    }

    pub fn get_payload_from_user_cookie_str(
        token: &str,
    ) -> jsonwebtoken::errors::Result<TokenData<UserJwtPayload>> {
        decode::<UserJwtPayload>(token, JWT_SECRET, &Validation::new(Algorithm::HS256))
    }
}
