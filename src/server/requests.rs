use super::types::*;

use rocket::request::{self, Request, FromRequest};
use rocket::http::Status;

use rocket::serde::Deserialize;

use chrono::{DateTime, Utc};
use chrono::serde::ts_seconds;

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
            None => request::Outcome::Failure((Status::Unauthorized, ()))
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct MatchClientRecord {
    #[serde(with = "ts_seconds")]
    start_time: DateTime<Utc>,
    game_id: GameType,
    cpu_level: CpuLevel,
    duration: i32,
    result: MatchResult
}

impl MatchClientRecord {
    pub fn unwrap_record(self) -> (DateTime<Utc>, GameType, CpuLevel, i32, MatchResult) {
        (self.start_time, self.game_id, self.cpu_level, self.duration, self.result)
    }
}
