#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_simpleauth;
extern crate toml;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate chan;
extern crate chan_signal;

mod auth;
mod config;
mod driver;
use self::rocket::response::Redirect;
use self::rocket_simpleauth::userpass::UserPass;
use chan_signal::Signal;
use config::Config;
use driver::{Driver, Motor};
use rocket::response::NamedFile;
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
        Driver {
            left: Motor::new(conf.left.pin1, conf.left.pin2),
            right: Motor::new(conf.right.pin1, conf.right.pin2),
        }
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

#[get("/driver/<op>")]
fn handle_driver(_info: UserPass<String>, op: String) -> Option<()> {
    println!("op:{}", op);
    match op.as_str() {
        "forward" => {
            DRV.forward();
            Some(())
        }
        "backward" => {
            DRV.backward();
            Some(())
        }
        "left" => {
            DRV.left();
            Some(())
        }
        "right" => {
            DRV.right();
            Some(())
        }
        "stop" => {
            DRV.stop();
            Some(())
        }
        _ => None,
    }
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
                handle_driver,
                handle_assets
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
