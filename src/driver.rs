extern crate rppal;

struct Motor {
    pin1 :u8, // IN1
    pin2 :u8, // IN2
}

impl Motor {
    fn new(p1 :u8, p2 :u8) -> Self {
        Gpio.set_mode(p1, Output);
        Gpio.set_mode(p2, Output);
        Motor{pin1 : p1, pin2 : p2}
    }

    fn forward(&self) {
        Gpio.write(self.pin1,HIGH);
        Gpio.write(self.pin2,LOW);
    }

    fn backward(&self) {
        Gpio.write(self.pin1,LOW);
        Gpio.write(self.pin2,HIGH);
    }

    fn stop(&self) {
        Gpio.write(self.pin1,LOW);
        Gpio.write(self.pin2,LOW);
    }
}

struct Driver {
    Motor left,
    Motor right,
}

impl Driver {
    fn forward(&self) {
        self.left.forward();
        self.right.forward();
    }

    fn backward(&self) {
        self.left.backward();
        self.right.backward();
    }

    fn left(&self) {
        self.left.forward();
        self.right.backward();
    }

    fn right(&self) {
        self.left.backward();
        self.right.forward();
    }
}
