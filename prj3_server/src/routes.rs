use rocket::http::{Cookie, CookieJar, Status};
use rocket::form::Form;

use rocket::serde::json::Json;

use super::UserDbConn;
use super::forms::*;

use shared_types::models::{UserModel, MatchRecordModel};
use shared_types::types::*;
use shared_types::queries::*;

#[post("/user/login", data="<auth>")]
async fn user_login(db: UserDbConn, auth: Form<UserAuthForm>, cookies: &CookieJar<'_>) -> Status {

    let (status, auth_cookie) = db.run(move |c| {
        match users::find_by_id(c, &auth.user_id) {
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
    use diesel::result::Error::DatabaseError;
    use diesel::result::DatabaseErrorKind;

    let uid = auth.user_id.clone();
    let auth = auth.into_inner();

    match UserModel::generate_new(auth.user_id, auth.password) {
        Ok(user) => {
            let (status, auth_cookie) = db.run(move |c| {
                match users::register_new(c, user) {
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
    db.run(move |c| {
        match_records::find_by_user(
            c,
            &auth.unwrap_token(),
            limit.unwrap_or(10),
            offset.unwrap_or(0)
        )
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
    use diesel::result::Error::DatabaseError;
    use diesel::result::DatabaseErrorKind;

    let match_record = MatchRecordModel::new_from_client(auth_token, record.into_inner());

    match db.run(move |c| {
        match_records::add(c, match_record)
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

    let sort_by = sort_by.unwrap_or(MatchQuerySortBy::StartTime);
    let asc = asc.unwrap_or_else(|| {
        match sort_by {
            MatchQuerySortBy::StartTime => false,
            MatchQuerySortBy::Duration => true
        }
    });

    db.run(move |c| {
        match_records::get(
            c,
            filter,
            sort_by,
            asc,
            before,
            after,
            limit.unwrap_or(10),
            offset.unwrap_or(0)
        )
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