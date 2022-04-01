#[cfg(feature = "diesel")]
use diesel::serialize::{self, Output, ToSql};
#[cfg(feature = "diesel")]
use diesel::deserialize::{self, FromSql};
#[cfg(feature = "diesel")]
use diesel::sql_types::Integer;

#[cfg(feature = "rocket")]
use rocket::request::{self, Request, FromRequest};
#[cfg(feature = "rocket")]
use rocket::http::Status;

#[cfg(feature = "diesel")]
use std::io::Write;

use serde::Serialize;
use serde::Deserialize;

use chrono::{DateTime, Utc};
use chrono::serde::ts_seconds;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "diesel", derive(FromSqlRow, AsExpression))]
#[cfg_attr(feature = "rocket", derive(FromFormField))]
#[cfg_attr(feature = "diesel", sql_type = "Integer")]
pub enum GameType {
    Connect4 = 1,
    OttoToot = 2
}

#[cfg(feature = "diesel")]
impl<DB> ToSql<Integer, DB> for GameType
where
    DB: diesel::backend::Backend,
    i32: ToSql<Integer, DB>,
{
    fn to_sql<W: Write>(&self, out: &mut Output<W, DB>) -> serialize::Result {
        (*self as i32).to_sql(out)
    }
}

#[cfg(feature = "diesel")]
impl<DB> FromSql<Integer, DB> for GameType
where
    DB: diesel::backend::Backend,
    i32: FromSql<Integer, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        match i32::from_sql(bytes)? {
            1 => Ok(GameType::Connect4),
            2 => Ok(GameType::OttoToot),
            x => Err(format!("Unrecognized GameType variant {}", x).into()),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "diesel", derive(FromSqlRow, AsExpression))]
#[cfg_attr(feature = "rocket", derive(FromFormField))]
#[cfg_attr(feature = "diesel", sql_type = "Integer")]
pub enum CpuLevel {
    Easy = 3,
    Medium = 6,
    Hard = 9
}

#[cfg(feature = "diesel")]
impl<DB> ToSql<Integer, DB> for CpuLevel
where
    DB: diesel::backend::Backend,
    i32: ToSql<Integer, DB>,
{
    fn to_sql<W: Write>(&self, out: &mut Output<W, DB>) -> serialize::Result {
        (*self as i32).to_sql(out)
    }
}

#[cfg(feature = "diesel")]
impl<DB> FromSql<Integer, DB> for CpuLevel
where
    DB: diesel::backend::Backend,
    i32: FromSql<Integer, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        match i32::from_sql(bytes)? {
            3 => Ok(CpuLevel::Easy),
            6 => Ok(CpuLevel::Medium),
            9 => Ok(CpuLevel::Hard),
            x => Err(format!("Unrecognized CpuLevel variant {}", x).into()),
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "diesel", derive(FromSqlRow, AsExpression))]
#[cfg_attr(feature = "rocket", derive(FromFormField))]
#[cfg_attr(feature = "diesel", sql_type = "Integer")]
pub enum MatchResult {
    Win = 1,
    Tie = 0,
    Loss = -1
}

#[cfg(feature = "diesel")]
impl<DB> ToSql<Integer, DB> for MatchResult
where
    DB: diesel::backend::Backend,
    i32: ToSql<Integer, DB>,
{
    fn to_sql<W: Write>(&self, out: &mut Output<W, DB>) -> serialize::Result {
        (*self as i32).to_sql(out)
    }
}

#[cfg(feature = "diesel")]
impl<DB> FromSql<Integer, DB> for MatchResult
where
    DB: diesel::backend::Backend,
    i32: FromSql<Integer, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        match i32::from_sql(bytes)? {
            1 => Ok(MatchResult::Win),
            0 => Ok(MatchResult::Tie),
            -1 => Ok(MatchResult::Loss),
            x => Err(format!("Unrecognized MatchResult variant {}", x).into()),
        }
    }
}

#[derive(Serialize)]
#[cfg_attr(feature = "rocket", derive(FromFormField))]
pub enum MatchQuerySortBy {
    StartTime,
    Duration,
}

#[cfg_attr(feature = "rocket", derive(FromForm))]
pub struct MatchQueryFilter {
    pub result: Vec<MatchResult>,
    pub game: Vec<GameType>,
    pub level: Vec<CpuLevel>
}

impl ToQueryPairs for MatchQueryFilter {
    type Output = (String, String);
    fn query_pairs(self) -> Vec<Self::Output> {
        use itertools::Itertools;
        self.result.into_iter()
        .unique()
        .map(|value| ("filter.result", match value {
            MatchResult::Loss => "loss",
            MatchResult::Tie => "tie",
            MatchResult::Win => "win"
        }))
        .chain(
            self.game.into_iter()
            .unique()
            .map(|value| ("filter.game", match value {
                GameType::Connect4 => "connect4",
                GameType::OttoToot => "ottotoot"
            }))
        )
        .chain(
            self.level.into_iter()
            .unique()
            .map(|value| ("filter.level", match value {
                CpuLevel::Easy => "easy",
                CpuLevel::Medium => "medium",
                CpuLevel::Hard => "hard"
            }))
        ).map(
            |(key, value)| (String::from(key), String::from(value))
        ).collect()
    }
}

#[derive(Debug)]
pub struct UserAuthToken(String);

impl UserAuthToken {
    pub fn unwrap_token(self) -> String {
        self.0
    }

}

#[cfg(feature = "manual_auth_token")]
impl From<String> for UserAuthToken {

    fn from(token: String) -> Self {
        UserAuthToken(token)
    }

}

#[cfg_attr(feature = "rocket", rocket::async_trait)]
#[cfg(feature = "rocket")]
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

#[derive(Debug, Deserialize, Serialize)]
pub struct MatchClientRecord {
    #[serde(with = "ts_seconds")]
    pub start_time: DateTime<Utc>,
    pub game_id: GameType,
    pub cpu_level: CpuLevel,
    pub duration: i32,
    pub result: MatchResult
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MatchRecord {
    pub user_id: String,
    #[serde(with = "ts_seconds")]
    pub start_time: DateTime<Utc>,
    pub game_id: GameType,
    pub cpu_level: CpuLevel,
    pub duration: i32,
    pub result: MatchResult
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "rocket", derive(FromForm))]
pub struct UserAuthForm {
    pub user_id: String,
    pub password: String,
}

pub trait ToQueryPairs {
    type Output: serde::Serialize;
    fn query_pairs(self) -> Vec<Self::Output>;
}