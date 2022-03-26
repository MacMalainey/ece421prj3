use diesel::serialize::{self, Output, ToSql};
use diesel::deserialize::{self, FromSql};
use diesel::sql_types::Integer;

use rocket::serde::Serialize;
use rocket::serde::Deserialize;

use std::io::Write;

#[derive(Debug, Clone, Copy, PartialEq, FromSqlRow, AsExpression, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Copy, PartialEq, FromSqlRow, AsExpression, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Copy, PartialEq, FromSqlRow, AsExpression, Serialize, Deserialize)]
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