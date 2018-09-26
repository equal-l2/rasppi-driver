#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate rocket_simpleauth;
extern crate toml;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate chan;
extern crate chan_signal;
extern crate serde_json;

mod auth;
mod config;
mod driver;
use chan_signal::Signal;
use config::Config;
use driver::{Driver, DriverState, Motor};
use rocket::http::{ContentType, Status};
use rocket::response::{Content, Failure, NamedFile, Redirect};
use rocket::Data;
use rocket_contrib::Json;
use rocket_simpleauth::userpass::UserPass;
use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};

lazy_static! {
    static ref DRV: Driver = {
        #[cfg(not(feature = "gpio"))]
        {
            println!("!!! WARNING : GPIO is disabled !!!");
        }
        let mut input = String::new();
        std::fs::File::open("GPIO.toml")
            .and_then(|mut f| f.read_to_string(&mut input))
            .expect("Could not read GPIO.toml");
        let conf: Config = toml::from_str(&input).expect("Bad structure in GPIO.toml");
        Driver::new(
            Motor::new(conf.left.pin1, conf.left.pin2),
            Motor::new(conf.right.pin1, conf.right.pin2),
        )
    };
}

#[get("/")]
fn handle_root() -> Redirect {
    Redirect::to("/admin")
}

#[get("/assets/<file..>")]
fn handle_assets(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("pages/assets/").join(file)).ok()
}

#[get("/hls/<file..>")]
fn handle_hls(_info: UserPass<String>, file: PathBuf) -> Option<Content<fs::File>> {
    let f = File::open(Path::new("pages/hls/").join(file.clone())).ok()?;
    let ct = match file.extension()?.to_str()? {
        "m3u8" => ContentType::new("application", "vnd.apple.mpegURL"),
        "m4s" => ContentType::new("video", "mp4"),
        "ts" => ContentType::new("video", "mp2t"),
        _ => return None,
    };
    Some(Content(ct, f))
}

#[derive(Deserialize, Serialize)]
struct StateContainer {
    state: DriverState,
}

#[put("/driver", data = "<state>")]
fn driver_put(_info: UserPass<String>, state: Data) -> Result<(), Failure> {
    let mut buf = String::new();
    let _ = state.open().read_to_string(&mut buf);
    if let Ok(inner) = serde_json::from_str::<StateContainer>(buf.as_str()) {
        let state = inner.state;
        DRV.change_state_to(state);
        Ok(())
    } else {
        Err(Failure(Status::BadRequest))
    }
}

#[get("/driver")]
fn driver_get(_info: UserPass<String>) -> Json<StateContainer> {
    Json(StateContainer {
        state: DRV.get_state(),
    })
}

fn run_server(_sdone: chan::Sender<()>) {
    rocket::ignite()
        .mount(
            "/",
            routes![
                auth::admin,
                auth::login,
                auth::login_post,
                auth::logout,
                auth::unauth,
                handle_root,
                handle_assets,
                handle_hls,
                driver_put,
                driver_get,
            ],
        ).launch();
}

fn main() {
    let signal = chan_signal::notify(&[Signal::INT, Signal::TERM]);
    let (sdone, rdone) = chan::sync(0);

    std::thread::spawn(move || run_server(sdone));

    chan_select! {
        signal.recv() -> _ => {
            DRV.cleanup();
        },
        rdone.recv() => {}
    }
}
