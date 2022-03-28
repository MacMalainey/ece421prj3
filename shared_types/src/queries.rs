use diesel::sqlite::SqliteConnection;

type Result<T> = std::result::Result<T, diesel::result::Error>;

#[cfg(feature = "run_migrations")]
embed_migrations!();

#[cfg(feature = "run_migrations")]
pub fn run_migrations(conn: &SqliteConnection) -> Result<()> {
    use diesel_migrations::RunMigrationsError;
    embedded_migrations::run_with_output(conn, &mut std::io::stdout()).map_err(
        |err| match err {
            RunMigrationsError::MigrationError(err) => panic!("DATABASE INITIALIZATION ERROR:\n{:?}", err),
            RunMigrationsError::QueryError(err) => err,
            RunMigrationsError::EmptyMigration => panic!("DATABASE INITIALIZATION ERROR:\n\tempty_migration"),
            _ => panic!("DATABASE INITIALIZATION ERROR:\n\tunknown error")
        }
    )
}

pub mod users {
    use diesel::prelude::*;
    use diesel::sqlite::SqliteConnection;

    use crate::models::UserModel;

    use super::Result;

    pub fn find_by_id(conn: &SqliteConnection, id: &str) -> Result<Option<UserModel>> {
        use crate::schema::users::dsl::*;

        users.find(id).first::<UserModel>(conn).optional()
    }

    pub fn register_new(conn: &SqliteConnection, user: UserModel) -> Result<()> {
        use crate::schema::users::dsl::*;

        user.insert_into(users).execute(conn).map(|_| ())
    }
}

pub mod match_records {
    use diesel::prelude::*;
    use diesel::sqlite::SqliteConnection;

    use chrono::NaiveDateTime;

    use crate::models::MatchRecordModel;
    use crate::types::{MatchQueryFilter, MatchQuerySortBy, MatchResult, GameType, CpuLevel};

    use super::Result;

    pub fn add(conn: &SqliteConnection, record: MatchRecordModel) -> Result<()> {
        use crate::schema::match_records::dsl::*;

        record.insert_into(match_records).execute(conn).map(|_| ())
    }

    pub fn find_by_user(conn: &SqliteConnection, uid: &str, limit: i64, offset: i64) -> Result<Vec<MatchRecordModel>> {
        use crate::schema::match_records::dsl::*;

        match_records
            .filter(user_id.eq(uid))
            .order_by(start_time.desc())
            .limit(limit)
            .offset(offset).load::<MatchRecordModel>(conn)
    }

    pub fn get(
        conn: &SqliteConnection,
        filter: Option<MatchQueryFilter>,
        sort_by: MatchQuerySortBy,
        asc: bool,
        before: Option<i64>,
        after: Option<i64>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<MatchRecordModel>> {
        use crate::schema::match_records::dsl::*;
        use itertools::Itertools;

        let mut query = match_records.into_boxed::<diesel::sqlite::Sqlite>();

        if let Some(filters) = filter {
            let result_filters: Vec<MatchResult> = filters.result.iter().unique().cloned().collect();
            if result_filters.len() > 0 {
                query = query.filter(result.eq_any(result_filters));
            }

            let game_filters: Vec<GameType> = filters.game.iter().unique().cloned().collect();
            if game_filters.len() > 0 {
                query = query.filter(game_id.eq_any(game_filters));
            }

            let level_filters: Vec<CpuLevel> = filters.level.iter().unique().cloned().collect();
            if level_filters.len() > 0 {
                query = query.filter(cpu_level.eq_any(level_filters));
            }
        }

        if let Some(before_ts) = before {
            query = query.filter(start_time.lt(NaiveDateTime::from_timestamp(before_ts, 0)));
        }

        if let Some(after_ts) = after {
            query = query.filter(start_time.gt(NaiveDateTime::from_timestamp(after_ts, 0)));
        }

        query = match sort_by {
            MatchQuerySortBy::StartTime =>
                if asc {
                    query.order(start_time.asc())
                } else {
                    query.order(start_time.desc())
                },
            MatchQuerySortBy::Duration =>
                if asc {
                    query.order((duration.asc(), start_time.desc()))
                } else {
                    query.order((duration.desc(), start_time.desc()))
                }
        };

        query = query.limit(limit).offset(offset);

        query.load::<MatchRecordModel>(conn)
    }
}