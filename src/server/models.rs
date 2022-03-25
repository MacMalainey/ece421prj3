use super::schema::*;
use super::requests::*;

use chrono::NaiveDateTime;
use chrono::naive::serde::ts_seconds;

use diesel::serialize::{self, Output, ToSql};
use diesel::deserialize::{self, FromSql};
use diesel::sql_types::Integer;

use rocket::serde::Serialize;

use std::io::Write;

#[derive(Debug, Identifiable, Queryable, Insertable)]
#[table_name = "users"]
#[primary_key(user_id)]
pub struct UserModel {
    pub user_id: String,
    pub password: String,
}

impl UserModel {

    pub fn new_from_form(form: UserAuthForm) -> Result<Self, argon2::Error> {
        // Generate password salt
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let mut salt = vec![0u8; form.user_id.len() + form.password.len()];
        salt.iter_mut().for_each(|val| *val = rng.gen());

        // Generate password hash
        let pwd_hash = argon2::hash_encoded(&form.password.as_bytes(), &salt, &argon2::Config::default())?;

        // Return model
        Ok(UserModel {
            user_id: form.user_id,
            password: pwd_hash
        })
    }

    pub fn compare(&self, password: &String) -> Result<bool, argon2::Error> {
        argon2::verify_encoded(&self.password, password.as_bytes())
    }

}

#[derive(Debug, Queryable, Insertable, Serialize)]
#[serde(crate = "rocket::serde")]
#[table_name = "match_records"]
pub struct MatchRecordModel {
    id: Option<i32>,
    user_id: String,
    #[serde(with = "ts_seconds")]
    start_time: NaiveDateTime,
    game_id: GameType,
    cpu_level: CpuLevel,
    duration: i32,
    result: MatchResult
}

#[derive(Debug, Clone, Copy, PartialEq, FromSqlRow, AsExpression, Serialize)]
#[serde(crate = "rocket::serde")]
#[sql_type = "Integer"]
pub enum GameType {
    Connect4 = 1,
    OttoToot = 2
}

impl<DB> ToSql<Integer, DB> for GameType
where
    DB: diesel::backend::Backend,
    i32: ToSql<Integer, DB>,
{
    fn to_sql<W: Write>(&self, out: &mut Output<W, DB>) -> serialize::Result {
        (*self as i32).to_sql(out)
    }
}

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

#[derive(Debug, Clone, Copy, PartialEq, FromSqlRow, AsExpression, Serialize)]
#[serde(crate = "rocket::serde")]
#[sql_type = "Integer"]
pub enum CpuLevel {
    Easy = 3,
    Medium = 6,
    Hard = 9
}

impl<DB> ToSql<Integer, DB> for CpuLevel
where
    DB: diesel::backend::Backend,
    i32: ToSql<Integer, DB>,
{
    fn to_sql<W: Write>(&self, out: &mut Output<W, DB>) -> serialize::Result {
        (*self as i32).to_sql(out)
    }
}

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

#[derive(Debug, Clone, Copy, PartialEq, FromSqlRow, AsExpression, Serialize)]
#[serde(crate = "rocket::serde")]
#[sql_type = "Integer"]
pub enum MatchResult {
    Win = 1,
    Tie = 0,
    Loss = -1
}

impl<DB> ToSql<Integer, DB> for MatchResult
where
    DB: diesel::backend::Backend,
    i32: ToSql<Integer, DB>,
{
    fn to_sql<W: Write>(&self, out: &mut Output<W, DB>) -> serialize::Result {
        (*self as i32).to_sql(out)
    }
}

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
