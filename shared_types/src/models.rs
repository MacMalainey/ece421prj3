/// Database models

use super::schema::*;
use super::types::*;

use chrono::NaiveDateTime;
use chrono::DateTime;
use chrono::Utc;

/// Database Model of a User
#[derive(Debug, Identifiable, Queryable, Insertable)]
#[table_name = "users"]
#[primary_key(user_id)]
pub struct UserModel {
    /// Username
    pub user_id: String,
    /// Hashed password
    pub password: String,
}

impl UserModel {

    /// Generate a UserModel from a user_id and raw_password
    /// 
    /// The generated model will have a hashed version of the raw_password
    /// 
    /// If the hash fails return the error
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

    /// Compares the given raw password with the hashed password
    /// 
    /// Returns true if the hashed password is the hash of the raw password
    pub fn compare(&self, password: &String) -> Result<bool, argon2::Error> {
        argon2::verify_encoded(&self.password, password.as_bytes())
    }

}

/// Database Model of a Match Record
#[derive(Debug, Queryable, Insertable)]
#[table_name = "match_records"]
pub struct MatchRecordModel {
    /// Database row ID
    id: Option<i32>,
    /// User ID for the record
    user_id: Option<String>,
    /// When the match was logged to the server
    finished_at: NaiveDateTime,
    /// Game played
    game_id: GameType,
    /// Level of opponent
    cpu_level: CpuLevel,
    /// Duration (in seconds) of match
    moves: i32,
    /// Result of match
    result: MatchResult
}

impl MatchRecordModel {

    /// Convert the model into a [MatchRecord]
    pub fn as_record(self) -> MatchRecord {
        MatchRecord {
            user_id: self.user_id,
            finished_at: DateTime::from_utc(self.finished_at, Utc),
            game_id: self.game_id,
            cpu_level: self.cpu_level,
            moves: self.moves,
            result: self.result
        }
    }

    /// Return the database ID of the model
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
            moves: record.moves,
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
            moves: record.moves,
            result: record.result
        }
    }
}