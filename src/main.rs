#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate toml;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

mod driver;
mod config;
use driver::{Driver, Motor};
use config::Config;
use std::io::Read;

lazy_static! {
    static ref DRV: Driver = {
        let mut input = String::new();
        std::fs::File::open("GPIO.toml").and_then(|mut f| f.read_to_string(&mut input)).expect("Could not read GPIO.toml");
        let conf: Config = toml::from_str(&input).expect("Bad structure in GPIO.toml");
        Driver {
            left: Motor::new(conf.left.pin1, conf.left.pin2),
            right: Motor::new(conf.right.pin1, conf.right.pin2),
        }
    };
}

#[get("/driver/<op>")]
fn handle_driver(op: String) -> Option<String> {
    println!("op:{}", op);
    match op.as_str() {
        "forward" => {
            DRV.forward();
            Some("".into())
        }
        "backward" => {
            DRV.backward();
            Some("".into())
        }
        "left" => {
            DRV.left();
            Some("".into())
        }
        "right" => {
            DRV.right();
            Some("".into())
        }
        "stop" => {
            DRV.stop();
            Some("".into())
        }
        _ => None,
    }
}

fn main() {
    rocket::ignite().mount("/", routes![handle_driver]).launch();
}
