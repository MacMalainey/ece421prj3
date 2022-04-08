use super::schema::*;
use super::types::*;

use chrono::NaiveDateTime;
use chrono::DateTime;
use chrono::Utc;

#[derive(Debug, Identifiable, Queryable, Insertable)]
#[table_name = "users"]
#[primary_key(user_id)]
pub struct UserModel {
    pub user_id: String,
    pub password: String,
}

impl UserModel {

    pub fn generate_new(user_id: String, raw_password: String) -> Result<Self, argon2::Error> {
        // Generate password salt
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let mut salt = vec![0u8; user_id.len() + raw_password.len()];
        salt.iter_mut().for_each(|val| *val = rng.gen());

        // Generate password hash
        let pwd_hash = argon2::hash_encoded(&raw_password.as_bytes(), &salt, &argon2::Config::default())?;

        // Return model
        Ok(UserModel {
            user_id: user_id,
            password: pwd_hash
        })
    }

    pub fn compare(&self, password: &String) -> Result<bool, argon2::Error> {
        argon2::verify_encoded(&self.password, password.as_bytes())
    }

}

#[derive(Debug, Queryable, Insertable)]
#[table_name = "match_records"]
pub struct MatchRecordModel {
    id: Option<i32>,
    user_id: Option<String>,
    finished_at: NaiveDateTime,
    game_id: GameType,
    cpu_level: CpuLevel,
    duration: i32,
    result: MatchResult
}

impl MatchRecordModel {

    pub fn as_record(self) -> MatchRecord {
        MatchRecord {
            user_id: self.user_id,
            finished_at: DateTime::from_utc(self.finished_at, Utc),
            game_id: self.game_id,
            cpu_level: self.cpu_level,
            duration: self.duration,
            result: self.result
        }
    }

    pub fn get_id(&self) -> Option<i32> {
        self.id
    }
}

impl From<(UserAuthToken, ClientMatchData)> for MatchRecordModel {
    
    fn from(f: (UserAuthToken, ClientMatchData)) -> Self {
        let (user_token, record) = f;
        MatchRecordModel {
            id: None,
            user_id: Some(user_token.into_inner()),
            finished_at: Utc::now().naive_utc(),
            game_id: record.game_id,
            cpu_level: record.cpu_level,
            duration: record.duration,
            result: record.result
        }
    }
}

impl From<(UserAuthToken, MatchRecord)> for MatchRecordModel {
    
    fn from(f: (UserAuthToken, MatchRecord)) -> Self {
        let (user_token, record) = f;
        MatchRecordModel {
            id: None,
            user_id: Some(user_token.into_inner()),
            finished_at: record.finished_at.naive_utc(),
            game_id: record.game_id,
            cpu_level: record.cpu_level,
            duration: record.duration,
            result: record.result
        }
    }
}