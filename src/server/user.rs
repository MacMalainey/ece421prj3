use rocket::http::{Cookie, CookieJar};
use rocket::form::Form;
use super::database::UserDbConn;

#[derive(FromForm)]
struct UserAuthData {
    user_id: String,
    password: String
}

#[post("/user/login", data="<auth>")]
fn user_login(db: UserDbConn, auth: Option<Form<UserAuthData>>, cookies: &CookieJar<'_>) -> &'static str {
    
    if let Some(data) = auth {
        cookies.add_private(
            Cookie::build("user_id", data.user_id.clone())
                .secure(true)
                .finish()
        );
    }

    "hi"

}

#[post("/user/logout")]
fn user_logout() -> &'static str {
    "HEY BUD"
}

#[post("/user/register")]
fn user_register() -> &'static str {
    "HEY BUD"
}

pub fn user_routes() -> Vec<rocket::Route> {
    routes![user_login, user_logout, user_register]
}