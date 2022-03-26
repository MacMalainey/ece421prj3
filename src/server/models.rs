use super::schema::*;
use super::requests::*;
use super::types::*;

use chrono::NaiveDateTime;
use chrono::naive::serde::ts_seconds;

use rocket::serde::Serialize;

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

impl MatchRecordModel {

    pub fn new_from_client(user_token: UserAuthToken, record: MatchClientRecord) -> Self {
        let (start_time_utc, game_id, cpu_level, duration, result) = record.unwrap_record();
        MatchRecordModel {
            id: None,
            user_id: user_token.unwrap_token(),
            start_time: start_time_utc.naive_utc(),
            game_id,
            cpu_level,
            duration,
            result
        }
    }
}
