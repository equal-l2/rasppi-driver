extern crate crypto;
extern crate rocket_simpleauth as rauth;

use self::rauth::status::{LoginRedirect, LoginStatus};
use self::rauth::userpass::UserPass;
use rocket::http::Cookies;
use rocket::request::Form;
use rocket::response::content::Html;
use rocket::response::Redirect;

//const ITERATION_COUNT: u32 = 10000;
const BP_USERNAME: &'static str =
    "$rpbkdf2$0$AAAnEA==$pdt8xlHGxk/GJa26Bg3g1Q==$VGt+AmV7OnN/trocjyDeJLDpmxShkhDsDSIfhIrQ+ws=$";
const BP_PASSWORD: &'static str =
    "$rpbkdf2$0$AAAnEA==$NdQsbZR5/aRAnLYGcXamZw==$gqVtt5IhhfIAe9os3QjCRNAyB1fkyQgsKeyRb/fERu0=$";

//fn hash_str(string: &str) -> Result<String, SimpleAuthenticator> {
//    crypto::pbkdf2::pbkdf2_simple(string, ITERATION_COUNT)
//        .or(Err(SimpleAuthenticator { username: None }))
//}

fn hash_cmp(plain: &str, hashed: &str) -> Result<bool, SimpleAuthenticator> {
    crypto::pbkdf2::pbkdf2_check(plain, hashed).or(Err(SimpleAuthenticator { username: None }))
}

pub struct SimpleAuthenticator {
    username: Option<String>,
}

impl rauth::authenticator::Authenticator for SimpleAuthenticator {
    type User = String;

    fn user(&self) -> String {
        self.username
            .as_ref()
            .map(|s| &**s)
            .unwrap_or("unknown")
            .into()
    }

    fn check_credentials(username: String, password: String) -> Result<Self, Self> {
        if hash_cmp(&username, BP_USERNAME)? && hash_cmp(&password, BP_PASSWORD)? {
            println!("[*] auth succeeded");
            Ok(SimpleAuthenticator {
                username: Some(username),
            })
        } else {
            println!("[*] auth failed");
            Err(SimpleAuthenticator { username: None })
        }
    }
}

#[get("/admin")]
pub fn admin(info: UserPass<String>) -> Html<String> {
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
    Html(
        "Authentication failed<br> \
         <a href=\"/admin\" >Login</a>"
            .into(),
    )
}

#[get("/logout")]
pub fn logout(mut info: UserPass<String>) -> Redirect {
    info.logout();
    Redirect::to("/admin")
}
