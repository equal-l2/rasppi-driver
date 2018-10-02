#[cfg(feature = "hls")]
use hls;
use std::io::Read;

lazy_static! {
    pub static ref CONF: Config = {
        #[cfg(not(feature = "gpio"))]
        {
            println!("!!! WARNING : GPIO is disabled !!!");
        }
        let mut input = String::new();
        std::fs::File::open("Config.toml")
            .and_then(|mut f| f.read_to_string(&mut input))
            .expect("Could not read Config.toml");
        toml::from_str(&input).expect("Bad structure in Config.toml")
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
