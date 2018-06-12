extern crate rppal;
use self::rppal::gpio::{Gpio, Level, Mode};
use std::sync::Mutex;

lazy_static! {
    static ref GPIO: Mutex<Gpio> = { Mutex::new(Gpio::new().unwrap()) };
}

pub struct Motor(u8, u8);

impl Motor {
    pub fn new(p1: u8, p2: u8) -> Self {
        let mut gpio = GPIO.lock().unwrap();
        (*gpio).set_mode(p1, Mode::Output);
        (*gpio).set_mode(p2, Mode::Output);
        Motor(p1, p2)
    }

    fn forward(&self) {
        println!("OUT: 1 0");
        let gpio = GPIO.lock().unwrap();
        (*gpio).write(self.0, Level::High);
        (*gpio).write(self.1, Level::Low);
    }

    fn backward(&self) {
        println!("OUT: 0 1");
        let gpio = GPIO.lock().unwrap();
        (*gpio).write(self.0, Level::Low);
        (*gpio).write(self.1, Level::High);
    }

    fn stop(&self) {
        println!("OUT: 0 0");
        let gpio = GPIO.lock().unwrap();
        (*gpio).write(self.0, Level::Low);
        (*gpio).write(self.1, Level::Low);
    }
}

pub struct Driver {
    pub left: Motor,
    pub right: Motor,
}

impl Driver {
    pub fn forward(&self) {
        self.left.forward();
        self.right.forward();
    }

    pub fn backward(&self) {
        self.left.backward();
        self.right.backward();
    }

    pub fn left(&self) {
        self.left.forward();
        self.right.backward();
    }

    pub fn right(&self) {
        self.left.backward();
        self.right.forward();
    }

    pub fn stop(&self) {
        self.left.stop();
        self.right.stop();
    }
}
