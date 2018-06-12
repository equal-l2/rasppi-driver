#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]
#[macro_use] extern crate rocket;
mod driver;
static driver :Driver = Driver{ left : driver::Motor::new(1,2), right : driver::Motor::new(3,4) };

#[get("/driver/<op>")]
fn begin(op: String) -> Option<String> {
    println!("op:{}", op);
    match op.as_str() {
        "forward" => { driver.forward(); Some("".into()) }
        "left" => { driver.left(); Some("".into()) }
        "right" => { driver.right(); Some("".into()) }
        "stop" => { driver.stop(); Some("".into()) }
        _ => None
    }
}

fn main() {
    rocket::ignite().mount("/", routes![web::begin]).launch();
}
