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
#[cfg(feature = "hls")]
mod hls;
use chan_signal::Signal;
use driver::{Driver, DriverState, IRType, Motor};
use rocket::http::Status;
use rocket::response::{Failure, NamedFile, Redirect};
use rocket::Data;
use rocket_contrib::Json;
use rocket_simpleauth::userpass::UserPass;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process;

lazy_static! {
    static ref DRV: Driver = {
        let conf = &config::CONF;
        Driver::new(
            Motor::new(conf.left.pin1, conf.left.pin2),
            Motor::new(conf.right.pin1, conf.right.pin2),
        )
    };
    static ref IR: IRType = {
        let conf = &config::CONF;
        IRType::new(conf.ir)
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

#[derive(Deserialize, Serialize)]
struct DriverStateContainer {
    state: DriverState,
}

#[put("/driver", data = "<state>")]
fn driver_put(_info: UserPass<String>, state: Data) -> Result<(), Failure> {
    let mut buf = String::new();
    let _ = state.open().read_to_string(&mut buf);
    if let Ok(inner) = serde_json::from_str::<DriverStateContainer>(buf.as_str()) {
        DRV.change_state_to(inner.state);
        Ok(())
    } else {
        Err(Failure(Status::BadRequest))
    }
}

#[get("/driver")]
fn driver_get(_info: UserPass<String>) -> Json<DriverStateContainer> {
    Json(DriverStateContainer {
        state: DRV.get_state(),
    })
}

#[derive(Deserialize, Serialize)]
struct IRStateContainer {
    state: bool,
}

#[put("/ir", data = "<state>")]
fn ir_put(_info: UserPass<String>, state: Data) -> Result<(), Failure> {
    let mut buf = String::new();
    let _ = state.open().read_to_string(&mut buf);
    if let Ok(inner) = serde_json::from_str::<IRStateContainer>(buf.as_str()) {
        IR.change_state_to(inner.state);
        Ok(())
    } else {
        Err(Failure(Status::BadRequest))
    }
}

#[get("/ir")]
fn ir_get(_info: UserPass<String>) -> Json<IRStateContainer> {
    Json(IRStateContainer {
        state: IR.get_state(),
    })
}

fn run_server(_sdone: chan::Sender<()>) {
    let mut routes = routes![
        auth::admin,
        auth::login,
        auth::login_post,
        auth::logout,
        auth::unauth,
        handle_root,
        handle_assets,
        driver_put,
        driver_get,
        ir_put,
        ir_get,
    ];

    #[cfg(feature = "hls")]
    {
        routes.append(&mut routes![hls::handle_hls]);
    }

    rocket::ignite().mount("/", routes).launch();
}

fn main() {
    lazy_static::initialize(&DRV);
    let signal = chan_signal::notify(&[Signal::INT, Signal::TERM]);
    let (sdone, rdone) = chan::sync(0);

    #[cfg(feature = "hls")]
    {
        std::thread::spawn(|| hls::run_hls().unwrap_or_else(|| process::exit(1)));
    }

    std::thread::spawn(move || run_server(sdone));

    chan_select! {
        signal.recv() -> _ => {
            driver::gpio_cleanup();
        },
        rdone.recv() => {}
    }
}
