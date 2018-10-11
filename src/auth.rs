extern crate pbkdf2;
extern crate rocket_simpleauth as rauth;

use self::pbkdf2::pbkdf2_check;
use self::rauth::status::{LoginRedirect, LoginStatus};
use self::rauth::userpass::UserPass;
use rocket::http::Cookies;
use rocket::request::Form;
use rocket::response::NamedFile;
use rocket::response::Redirect;
use std::io;

//const ITERATION_COUNT: u32 = 10000;
const BP_USERNAME: &str =
    "$rpbkdf2$0$AAAnEA==$pdt8xlHGxk/GJa26Bg3g1Q==$VGt+AmV7OnN/trocjyDeJLDpmxShkhDsDSIfhIrQ+ws=$";
const BP_PASSWORD: &str =
    "$rpbkdf2$0$AAAnEA==$NdQsbZR5/aRAnLYGcXamZw==$gqVtt5IhhfIAe9os3QjCRNAyB1fkyQgsKeyRb/fERu0=$";
const STUDENT_CARD_SYS_CODE: &str =
    "$rpbkdf2$0$AAAnEA==$pLQF6zHBosE7EKFTs7eXeQ==$FEFgc8QmU8RoSekBlG1tF+L0roRmQdA+9DDRs91A2F4=$";

//fn hash_str(string: &str) -> Result<String, SimpleAuthenticator> {
//    pbkdf2::pbkdf2_simple(string, ITERATION_COUNT)
//        .or(Err(SimpleAuthenticator { username: None }))
//}

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
        if pbkdf2_check(&username, STUDENT_CARD_SYS_CODE).is_ok() || (pbkdf2_check(&username, BP_USERNAME).is_ok() && pbkdf2_check(&password, BP_PASSWORD).is_ok())
        {
            println!("[auth] auth succeeded");
            Ok(SimpleAuthenticator {
                username: Some(username),
            })
        } else {
            println!("[auth] auth failed");
            Err(SimpleAuthenticator { username: None })
        }
    }
}

#[get("/admin")]
pub fn admin(_info: UserPass<String>) -> io::Result<NamedFile> {
    NamedFile::open("pages/control.html")
}

#[get("/admin", rank = 2)]
pub fn login() -> io::Result<NamedFile> {
    NamedFile::open("pages/login.html")
}

#[post("/admin", data = "<form>")]
pub fn login_post(form: Form<LoginStatus<SimpleAuthenticator>>, cookies: Cookies) -> LoginRedirect {
    form.into_inner().redirect("/admin", "/unauth", cookies)
}

#[get("/unauth")]
pub fn unauth() -> io::Result<NamedFile> {
    NamedFile::open("pages/unauth.html")
}

#[get("/logout")]
pub fn logout(mut info: UserPass<String>) -> Redirect {
    info.logout();
    Redirect::to("/admin")
}
