#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate toml;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

mod config;
mod driver;
use config::Config;
use driver::{Driver, Motor};
use std::io::Read;

lazy_static! {
    static ref DRV: Driver = {
        #[cfg(not(feature = "gpio"))] {
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

#[get("/driver/<op>")]
fn handle_driver(op: String) -> Option<()> {
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

fn main() {
    rocket::ignite().mount("/", routes![handle_driver]).launch();
}
