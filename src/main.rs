#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]
extern crate rocket;
#[macro_use]
extern crate lazy_static;
mod driver;
use driver::{Driver, Motor};

lazy_static! {
    static ref DRV: Driver = {
        Driver {
            left: Motor::new(19, 26),
            right: Motor::new(20, 21),
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
