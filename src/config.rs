#[cfg(feature = "hls")]
use hls;
use std::io::Read;

lazy_static! {
    pub static ref CONF: Config = {
        #[cfg(not(feature = "gpio"))]
        {
            println!("[config] GPIO disabled");
        }
        let input =
            std::fs::read_to_string("Config.toml").expect("[config] Could not read Config.toml");
        toml::from_str(&input).expect("[config] Bad structure in Config.toml")
    };
}

#[derive(Deserialize)]
pub struct Config {
    pub left: Pins,
    pub right: Pins,
    pub ir: u8,
    #[cfg(feature = "hls")]
    pub hls: hls::HLSConfig,
}

#[derive(Deserialize)]
pub struct Pins {
    pub pin1: u8,
    pub pin2: u8,
}
