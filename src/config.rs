#[derive(Deserialize)]
pub struct Config {
    pub left: Pins,
    pub right: Pins,
}

#[derive(Deserialize)]
pub struct Pins {
    pub pin1: u8,
    pub pin2: u8,
}
