use std::{thread, time};
#[cfg(feature = "gpio")]
extern crate rppal;
#[cfg(feature = "gpio")]
use self::rppal::gpio::{Gpio, Level, Mode};
#[cfg(feature = "gpio")]
use std::sync::Mutex;

#[cfg(feature = "gpio")]
lazy_static! {
    static ref GPIO: Mutex<Gpio> = { Mutex::new(Gpio::new().unwrap()) };
}

pub struct Motor(u8, u8);

impl Motor {
    pub fn new(p1: u8, p2: u8) -> Self {
        #[cfg(feature = "gpio")]
        {
            let mut gpio = GPIO.lock().unwrap();
            (*gpio).set_mode(p1, Mode::Output);
            (*gpio).set_mode(p2, Mode::Output);
        }
        println!("GPIO init: {} {}", p1, p2);
        Motor(p1, p2)
    }

    fn forward(&self) {
	self.stop();
	thread::sleep(time::Duration::from_millis(1));
        #[cfg(feature = "gpio")]
        {
            let gpio = GPIO.lock().unwrap();
            (*gpio).write(self.0, Level::High);
            (*gpio).write(self.1, Level::Low);
        }
        println!("GPIO {} {} : 1 0", self.0, self.1);
    }

    fn backward(&self) {
	self.stop();
	thread::sleep(time::Duration::from_millis(1));
        #[cfg(feature = "gpio")]
        {
            let gpio = GPIO.lock().unwrap();
            (*gpio).write(self.0, Level::Low);
            (*gpio).write(self.1, Level::High);
        }
        println!("GPIO {} {} : 0 1", self.0, self.1);
    }

    fn stop(&self) {
        #[cfg(feature = "gpio")]
        {
            let gpio = GPIO.lock().unwrap();
            (*gpio).write(self.0, Level::Low);
            (*gpio).write(self.1, Level::Low);
        }
        println!("GPIO {} {} : 0 0", self.0, self.1);
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

    pub fn cleanup(&self) {
        #[cfg(feature = "gpio")]
        {
            let mut gpio = GPIO.lock().unwrap();
            (*gpio).cleanup();
        }
	println!("GPIO Clean Up");
    }
}
