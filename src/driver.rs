use std::thread;
use std::time::{self, Duration};
#[cfg(feature = "gpio")]
extern crate rppal;
#[cfg(feature = "gpio")]
use self::rppal::gpio::{Gpio, Level, Mode};
use std::cell::Cell;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

#[cfg(feature = "gpio")]
lazy_static! {
    static ref GPIO: Mutex<Gpio> = { Mutex::new(Gpio::new().unwrap()) };
}

pub fn gpio_cleanup() {
    #[cfg(feature = "gpio")]
    {
        let mut gpio = GPIO.lock().unwrap();
        (*gpio).cleanup();
    }
    println!("[driver] GPIO Clean Up");
}

const WAIT: Duration = time::Duration::from_micros(100);

pub struct Motor(u8, u8);

impl Motor {
    pub fn new(p1: u8, p2: u8) -> Self {
        #[cfg(feature = "gpio")]
        {
            let mut gpio = GPIO.lock().unwrap();
            (*gpio).set_mode(p1, Mode::Output);
            (*gpio).set_mode(p2, Mode::Output);
        }
        println!("[driver] Motor GPIO init: {} {}", p1, p2);
        Motor(p1, p2)
    }

    fn forward(&self) {
        self.stop();
        thread::sleep(WAIT);
        #[cfg(feature = "gpio")]
        {
            let gpio = GPIO.lock().unwrap();
            (*gpio).write(self.0, Level::High);
            (*gpio).write(self.1, Level::Low);
        }
        println!("[driver] Motor GPIO {} {} : 1 0", self.0, self.1);
    }

    fn backward(&self) {
        self.stop();
        thread::sleep(WAIT);
        #[cfg(feature = "gpio")]
        {
            let gpio = GPIO.lock().unwrap();
            (*gpio).write(self.0, Level::Low);
            (*gpio).write(self.1, Level::High);
        }
        println!("[driver] Motor GPIO {} {} : 0 1", self.0, self.1);
    }

    fn stop(&self) {
        #[cfg(feature = "gpio")]
        {
            let gpio = GPIO.lock().unwrap();
            (*gpio).write(self.0, Level::Low);
            (*gpio).write(self.1, Level::Low);
        }
        println!("[driver] Motor GPIO {} {} : 0 0", self.0, self.1);
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum DriverState {
    Forward,
    Backward,
    Right,
    Left,
    Stop,
}

pub struct Driver {
    pub left: Motor,
    pub right: Motor,
    state: Mutex<Cell<DriverState>>,
}

impl Driver {
    pub fn new(m1: Motor, m2: Motor) -> Self {
        Driver {
            left: m1,
            right: m2,
            state: Mutex::new(Cell::new(DriverState::Stop)),
        }
    }

    pub fn change_state_to(&self, stat: DriverState) {
        match stat {
            DriverState::Forward => {
                self.forward();
            }
            DriverState::Backward => {
                self.backward();
            }
            DriverState::Right => {
                self.right();
            }
            DriverState::Left => {
                self.left();
            }
            DriverState::Stop => {
                self.stop();
            }
        }
    }

    fn set_state(&self, st: DriverState) {
        let cell = self.state.lock().unwrap();
        println!(
            "[driver] Driver State change : {:?} -> {:?}",
            cell.get(),
            st
        );
        cell.set(st);
    }

    pub fn get_state(&self) -> DriverState {
        self.state.lock().unwrap().get()
    }

    pub fn forward(&self) {
        self.left.forward();
        self.right.forward();
        self.set_state(DriverState::Forward);
    }

    pub fn backward(&self) {
        self.left.backward();
        self.right.backward();
        self.set_state(DriverState::Backward);
    }

    pub fn left(&self) {
        self.left.forward();
        self.right.backward();
        self.set_state(DriverState::Left);
    }

    pub fn right(&self) {
        self.left.backward();
        self.right.forward();
        self.set_state(DriverState::Right);
    }

    pub fn stop(&self) {
        self.left.stop();
        self.right.stop();
        self.set_state(DriverState::Stop);
    }
}

pub struct IRType {
    pin: u8,
    state: AtomicBool,
}

impl IRType {
    pub fn new(p: u8) -> Self {
        #[cfg(feature = "gpio")]
        {
            let mut gpio = GPIO.lock().unwrap();
            (*gpio).set_mode(p, Mode::Output);
        }
        println!("[driver] IR GPIO init: {}", p);
        IRType {
            pin: p,
            state: AtomicBool::new(false),
        }
    }

    pub fn change_state_to(&self, s: bool) {
        if s {
            self.on();
        } else {
            self.off();
        }
    }

    fn set_state(&self, s: bool) {
        self.state.store(s, Ordering::SeqCst);
    }

    pub fn get_state(&self) -> bool {
        self.state.load(Ordering::SeqCst)
    }

    fn on(&self) {
        #[cfg(feature = "gpio")]
        {
            let gpio = GPIO.lock().unwrap();
            (*gpio).write(self.pin, Level::High);
        }
        self.set_state(true);
        println!("[driver] IR GPIO {} : 1", self.pin);
    }

    fn off(&self) {
        #[cfg(feature = "gpio")]
        {
            let gpio = GPIO.lock().unwrap();
            (*gpio).write(self.pin, Level::Low);
        }
        self.set_state(false);
        println!("[driver] IR GPIO {} : 0", self.pin);
    }
}
