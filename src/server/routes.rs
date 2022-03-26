use rocket::http::{Cookie, CookieJar, Status};
use rocket::serde::json::Json;
use rocket::form::Form;

use diesel::prelude::*;

use chrono::NaiveDateTime;

use super::database::UserDbConn;
use super::models::*;
use super::requests::*;
use super::types::*;

#[post("/user/login", data="<auth>")]
async fn user_login(db: UserDbConn, auth: Form<UserAuthForm>, cookies: &CookieJar<'_>) -> Status {

    use super::schema::users::dsl::*;

    let (status, auth_cookie) = db.run(move |c| {
        match users.find(&auth.user_id).first::<UserModel>(c).optional() {
            Ok(Some(user)) => {
                match user.compare(&auth.password) {
                    Ok(true) => {
                        (Status::Ok, Some(Cookie::build("user_auth_token", user.user_id).finish()))
                    },
                    Ok(false) => {
                        (Status::Unauthorized, None)
                    },
                    Err(err) => {
                        eprintln!("{:?}", err);
                        (Status::InternalServerError, None)
                    }
                }
            },
            Ok(None) => {
                (Status::Unauthorized, None)
            }
            Err(err) => {
                eprintln!("{:?}", err);
                (Status::InternalServerError, None)
            }
        }
    }).await;

    if let Some(cookie) = auth_cookie {
        cookies.add_private(cookie);
    }

    status
}

#[post("/user/logout")]
fn user_logout(cookies: &CookieJar<'_>, _auth: UserAuthToken) -> Status {
    cookies.remove_private(Cookie::named("user_auth_token"));
    Status::Ok
}

#[post("/user/register", data="<auth>")]
async fn user_register(db: UserDbConn, auth: Form<UserAuthForm>, cookies: &CookieJar<'_>) -> Status {
    use super::schema::users::dsl::*;
    use diesel::result::Error::DatabaseError;
    use diesel::result::DatabaseErrorKind;

    let uid = auth.user_id.clone();

    match UserModel::new_from_form(auth.into_inner()) {
        Ok(user) => {
            let (status, auth_cookie) = db.run(move |c| {
                match user.insert_into(users).execute(c) {
                    Ok(_) => {
                        (Status::Ok, Some(Cookie::build("user_auth_token", uid).finish()))
                    },
                    Err(DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => (Status::Unauthorized, None),
                    Err(err) => {
                        eprintln!("{:?}", err);
                        (Status::InternalServerError, None)
                    }
                }
            }).await;
        
            if let Some(cookie) = auth_cookie {
                cookies.add_private(cookie);
            }
        
            status
        },
        Err(err) => {
            eprintln!("{:?}", err);
            Status::InternalServerError
        }
    }

}

#[get("/user/records?<limit>&<offset>")]
async fn user_records(db: UserDbConn, auth: UserAuthToken, limit: Option<i64>, offset: Option<i64>) -> Result<Json<Vec<MatchRecordModel>>, Status> {
    use super::schema::match_records::dsl::*;

    db.run(move |c| {
        match_records
            .filter(user_id.eq(auth.unwrap_token()))
            .order_by(start_time.desc())
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0)).load::<MatchRecordModel>(c)
    }).await
        .map(|data| Json(data))
        .map_err(|err| {
            eprintln!("{:?}", err);
            Status::InternalServerError
        }
    )
}

#[post("/user/records/add", format = "json", data = "<record>",)]
async fn user_record_add(db: UserDbConn, record: Json<MatchClientRecord>, auth_token: UserAuthToken, cookies: &CookieJar<'_>) -> Status {
    use super::schema::match_records::dsl::*;
    use diesel::result::Error::DatabaseError;
    use diesel::result::DatabaseErrorKind;

    let match_record = MatchRecordModel::new_from_client(auth_token, record.into_inner());

    match db.run(move |c| {
        match_record.insert_into(match_records).execute(c)
    }).await {
        Ok(_) => Status::Ok,
        Err(DatabaseError(DatabaseErrorKind::ForeignKeyViolation, _)) => {
            // We encountered a user that doesn't actually exist
            cookies.remove_private(Cookie::named("user_auth_token"));
            Status::Unauthorized
        },
        Err(_) => {
            Status::InternalServerError
        }
    }
}

#[get("/games/records?<limit>&<offset>&<before>&<after>&<sort_by>&<asc>&<filter>")]
async fn game_records(
    db: UserDbConn,
    limit: Option<i64>,
    offset: Option<i64>,
    before: Option<i64>,
    after: Option<i64>,
    sort_by: Option<MatchQuerySortBy>,
    asc: Option<bool>,
    filter: Option<MatchQueryFilter>
) -> Result<Json<Vec<MatchRecordModel>>, Status> {
    use super::schema::match_records::dsl::*;
    use itertools::Itertools;

    db.run(move |c| {
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

        query = match sort_by.unwrap_or(MatchQuerySortBy::StartTime) {
            MatchQuerySortBy::StartTime =>
                if asc.unwrap_or(false) {
                    query.order(start_time.asc())
                } else {
                    query.order(start_time.desc())
                },
            MatchQuerySortBy::Duration =>
                if asc.unwrap_or(true) {
                    query.order((duration.asc(), start_time.desc()))
                } else {
                    query.order((duration.desc(), start_time.desc()))
                }
        };

        query = query.limit(limit.unwrap_or(10)).offset(offset.unwrap_or(0));

        query.load::<MatchRecordModel>(c)
    }).await
        .map(|data| Json(data))
        .map_err(|err| {
            eprintln!("{:?}", err);
            Status::InternalServerError
        }
    )
}

pub fn get_routes() -> Vec<rocket::Route> {
    routes![
        user_login,
        user_logout,
        user_register,
        user_records,
        user_record_add,
        game_records,
    ]
}