use rocket::http::{Cookie, CookieJar, Status};
use rocket::form::Form;

use rocket::serde::json::Json;

use super::UserDbConn;

use shared_types::models::{UserModel, MatchRecordModel};
use shared_types::types::*;
use shared_types::queries::*;

/// User Login Route
/// 
/// The route authenticates a user using form data
/// 
/// On Success
///  - Returns status 200
///  - Sets user_auth_form cookie as encrypted value
#[post("/user/login", data="<auth>")]
async fn user_login(db: UserDbConn, auth: Form<UserAuthForm>, cookies: &CookieJar<'_>) -> Status {

    let (status, auth_cookie) = db.run(move |c| {
        match users::find_by_id(c, &auth.user_id) {
            // Find user
            Ok(Some(user)) => {
                // Verify password
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

    // Set user_auth_token
    if let Some(cookie) = auth_cookie {
        cookies.add_private(cookie);
    }

    status
}

/// User Verify Route
/// 
/// Checks the value of the user_auth_token cookie and returns the user it is associated with
/// 
/// On Success:
///  - Returns status 200
///  - Returns JSON serialization of UserInfo type
#[get("/user/verify")]
async fn user_verify(
    db: UserDbConn,
    cookies: &CookieJar<'_>,
    auth: UserAuthToken
) -> Result<Json<UserInfo>, Status> {
    let user_id = auth.into_inner();
    let user_id_moved = user_id.clone();

    let result = db.run(move |c| 
        users::find_by_id(c, &user_id_moved).map(|user| user.is_some())
    ).await;

    match result {
        Ok(true) => Ok(Json(UserInfo { user_id })),
        Ok(false) => {
            cookies.remove_private(Cookie::named("user_auth_token"));
            Err(Status::Unauthorized)
        }
        Err(err) => {
            eprintln!("{:?}", err);
            Err(Status::InternalServerError)
        }
    }
}

/// User Logout Route
/// 
/// Deletes the user_auth_token cookie effectively "logging" the user out
/// 
/// Always returns status 200
#[post("/user/logout")]
fn user_logout(cookies: &CookieJar<'_>, _auth: UserAuthToken) -> Status {
    cookies.remove_private(Cookie::named("user_auth_token"));
    Status::Ok
}

/// User Register Route
/// 
/// Registers a new user given the provided form data
/// 
/// On Success
///  - Return status 200
///  - Set user_auth_token cookie (see User Login)
#[post("/user/register", data="<auth>")]
async fn user_register(db: UserDbConn, auth: Form<UserAuthForm>, cookies: &CookieJar<'_>) -> Status {
    use diesel::result::Error::DatabaseError;
    use diesel::result::DatabaseErrorKind;

    let uid = auth.user_id.clone();
    let auth = auth.into_inner();

    match UserModel::generate_new(auth.user_id, auth.password) {
        Ok(user) => {
            let (status, auth_cookie) = db.run(move |c| {
                match users::add(c, user) {
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

/// List User Records Route
/// 
/// Query the records for the authenticated user
/// 
/// For query arg descriptions check the design doc
/// 
/// On Success:
///  - Return Status 200
///  - Return JSON serialized [MatchRecord] list
#[get("/user/records?<limit>&<offset>&<before>&<after>&<sort_by>&<asc>&<filter>")]
async fn user_records(
    db: UserDbConn,
    auth: UserAuthToken,
    limit: Option<i64>,
    offset: Option<i64>,
    before: Option<i64>,
    after: Option<i64>,
    sort_by: Option<MatchQuerySortBy>,
    asc: Option<bool>,
    filter: Option<MatchQueryFilter>
) -> Result<Json<Records<MatchRecord>>, Status> {
    // Get sort options (or use defaults)
    let sort_by = sort_by.unwrap_or(MatchQuerySortBy::StartTime);
    let asc = asc.unwrap_or_else(|| {
        match sort_by {
            MatchQuerySortBy::StartTime => false,
            MatchQuerySortBy::Duration => true
        }
    });

    // Get offset if any
    let offset = offset.unwrap_or(0);

    db.run(move |c| {
        match_records::find_by_user(
            c,
            &auth.into_inner(),
            filter,
            sort_by,
            asc,
            before,
            after,
            limit.unwrap_or(10),
            offset
        )
    }).await
        .map(|mut data| Json(
            Records {
                records: data.0.drain(..).map(|r| r.as_record()).collect(),
                offset,
                total_count: data.1
            }
        ))
        .map_err(|err| {
            eprintln!("{:?}", err);
            Status::InternalServerError
        }
    )
}

/// Add a record for a given user
/// 
/// Submits a record for the authenticated user using JSON request body data
/// 
/// On Success:
///  - Return Status 200
#[post("/user/records/add", format = "json", data = "<record>",)]
async fn user_record_add(db: UserDbConn, record: Json<ClientMatchData>, auth_token: UserAuthToken, cookies: &CookieJar<'_>) -> Status {
    use diesel::result::Error::DatabaseError;
    use diesel::result::DatabaseErrorKind;

    let match_record = MatchRecordModel::from((auth_token, record.into_inner()));

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

/// List Game Records Route
/// 
/// Query game records
/// 
/// For query arg descriptions check the design doc
/// 
/// On Success:
///  - Return Status 200
///  - Return JSON serialized [MatchRecord] list
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
) -> Result<Json<Records<MatchRecord>>, Status> {

    // Get sort options (or use defaults)
    let sort_by = sort_by.unwrap_or(MatchQuerySortBy::StartTime);
    let asc = asc.unwrap_or_else(|| {
        match sort_by {
            MatchQuerySortBy::StartTime => false,
            MatchQuerySortBy::Duration => true
        }
    });

    // Get offset if any
    let offset = offset.unwrap_or(0);

    db.run(move |c| {
        match_records::find_all_users(
            c,
            filter,
            sort_by,
            asc,
            before,
            after,
            limit.unwrap_or(10),
            offset
        )
    }).await
        .map(|mut data| Json(
            Records {
                records: data.0.drain(..).map(|r| r.as_record()).collect(),
                offset,
                total_count: data.1
            }
        ))
        .map_err(|err| {
            eprintln!("{:?}", err);
            Status::InternalServerError
        }
    )
}

/// Returns all the routes to serve
pub fn get_routes() -> Vec<rocket::Route> {
    routes![
        user_login,
        user_verify,
        user_logout,
        user_register,
        user_records,
        user_record_add,
        game_records,
    ]
}