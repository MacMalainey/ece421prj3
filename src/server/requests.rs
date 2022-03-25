use rocket::request::{self, Request, FromRequest};

#[derive(Debug, FromForm)]
pub struct UserAuthForm {
    pub user_id: String,
    pub password: String,
}

#[derive(Debug)]
pub struct UserAuthToken(String);

impl UserAuthToken {
    pub fn unwrap_token(self) -> String {
        self.0
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserAuthToken {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        match req.cookies().get_private("user_auth_token") {
            Some(cookie) => request::Outcome::Success(
                UserAuthToken(String::from(cookie.value()))
            ),
            None => request::Outcome::Forward(())
        }
    }
}