use rocket::http::{Cookie, CookieJar, Status};
use rocket::form::Form;

use diesel::prelude::*;

use super::database::UserDbConn;
use super::models::*;
use super::forms::*;

#[post("/user/login", data="<auth>")]
async fn user_login(db: UserDbConn, auth: Form<UserAuthForm>, cookies: &CookieJar<'_>) -> Status {

    use super::schema::users::dsl::*;

    let (status, auth_cookie) = db.run(move |c| {
        match users.find(&auth.user_id).first::<UserModel>(c).optional() {
            Ok(Some(user)) => {
                match user.compare(&auth.password) {
                    Ok(true) => {
                        (Status::Ok, Some(Cookie::build("user_id", user.user_id).finish()))
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
fn user_logout(cookies: &CookieJar<'_>) -> Status {
    cookies.remove_private(Cookie::named("user_id"));
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
                        (Status::Ok, Some(Cookie::build("user_id", uid).finish()))
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

pub fn get_routes() -> Vec<rocket::Route> {
    routes![user_login, user_logout, user_register]
}