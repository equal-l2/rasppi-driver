use config;
use rocket::http::ContentType;
use rocket::response::Content;
use rocket_simpleauth::userpass::UserPass;
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Deserialize)]
pub struct HLSConfig {
    cmd: String, // commands to generate HLS stream
    path: String, // working directory for cmd and directory for placing HLS files
}

pub fn run_hls() {
    let wd = Path::new(&config::CONF.hls.path);
    if (!wd.is_dir()) {
        panic!("{}: not found or not a directory", config::CONF.hls.path);
    }
    let cmd: Vec<_> = config::CONF.hls.cmd.split(' ').collect();
    let mut child = Command::new(cmd[0])
        .args(&cmd[1..])
        .current_dir(wd)
        .spawn()
        .unwrap();
    child.wait();
}

#[get("/hls/<file..>")]
pub fn handle_hls(_info: UserPass<String>, file: PathBuf) -> Option<Content<fs::File>> {
    let f = File::open(Path::new(&config::CONF.hls.path).join(file.clone())).ok()?;
    let ct = match file.extension()?.to_str()? {
        "m3u8" => ContentType::new("application", "vnd.apple.mpegURL"),
        "m4s" | "mp4" => ContentType::new("video", "mp4"),
        "ts" => ContentType::new("video", "MP2T"),
        _ => return None,
    };
    Some(Content(ct, f))
}
