extern crate rocket_simpleauth as rauth;

use self::rauth::status::{LoginRedirect, LoginStatus};
use self::rauth::userpass::UserPass;
use rocket::http::Cookies;
use rocket::request::Form;
use rocket::response::content::Html;
use rocket::response::Redirect;

pub struct SimpleAuthenticator;

impl rauth::authenticator::Authenticator for SimpleAuthenticator {
    type User = String;

    fn user(&self) -> String {
        "its".to_string()
    }

    fn check_credentials(username: String, passwd: String) -> Result<Self, Self> {
        if username == "its" && passwd == "ume2018" {
            println!("[*] auth succeeded");
            Ok(SimpleAuthenticator)
        } else {
            println!("[*] auth failed");
            Err(SimpleAuthenticator)
        }
    }
}

#[get("/admin")]
pub fn admin(info: UserPass<String>) -> Html<String> {
    // we use request guards to fall down to the login page if UserPass couldn't find a valid cookie
    Html(format!(
        "Authentication succeeded: user logged in: {}<br>\
<a href=\"/driver/forward\" >Forward</a>
<a href=\"/driver/backward\" >Backward</a>
<a href=\"/driver/left\" >Left</a>
<a href=\"/driver/right\" >Right</a>
<a href=\"/driver/stop\" >Stop</a>
<a href=\"/logout\" >Logout</a>",
        info.user
    ))
}

#[get("/admin", rank = 2)]
pub fn login() -> Html<&'static str> {
    Html(
        "Authentication required to operate
    <form action=\"/admin\" method=\"POST\"> \
        <input type=\"text\" name=\"username\" /> \
        <input type=\"password\" name=\"password\" /> \
        <input type=\"submit\" value=\"Login\" /> \
    </form>",
    )
}

#[post("/admin", data = "<form>")]
pub fn login_post(form: Form<LoginStatus<SimpleAuthenticator>>, cookies: Cookies) -> LoginRedirect {
    form.into_inner().redirect("/admin", "/unauth", cookies)
}

#[get("/unauth")]
pub fn unauth() -> Html<String> {
    // we use request guards to fall down to the login page if UserPass couldn't find a valid cookie
    Html("Authentication failed".into())
}

#[get("/logout")]
pub fn logout(mut info: UserPass<String>) -> Redirect {
    info.logout();
    Redirect::to("/admin")
}
