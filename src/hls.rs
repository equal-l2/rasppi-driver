use config;
use rocket::http::ContentType;
use rocket::response::Content;
use rocket_simpleauth::userpass::UserPass;
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Deserialize)]
pub struct HLSConfig {
    cmd: String,  // commands to generate HLS stream
    path: String, // working directory for cmd and directory for placing HLS files
}

pub fn run_hls() -> Option<()> {
    let path = &config::CONF.hls.path;
    println!("[hls] FFmpeg destination path: {}", path);
    println!("[hls] FFmpeg command: {:?}", config::CONF.hls.cmd);
    let wd = Path::new(path);
    if !wd.is_dir() {
        panic!("[hls] {}: not found or not a directory", path);
    }
    let cmd: Vec<_> = config::CONF.hls.cmd.split(' ').collect();
    let f = File::create("ffmpeg.log").ok()?;
    let status = Command::new(cmd[0])
        .args(&cmd[1..])
        .stdout(f.try_clone().unwrap())
        .stderr(f)
        .current_dir(wd)
        .status();

    if status.is_err() {
        println!("[hls] Fatal : FFmpeg could not run");
        println!("[hls] Review cmd in Config.toml");
        return None;
    }

    if !status.unwrap().success() {
        println!("[hls] Fatal : FFmpeg exited unsuccessfully");
        println!("[hls] Review cmd in Config.toml");
        return None;
    }

    Some(())
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
