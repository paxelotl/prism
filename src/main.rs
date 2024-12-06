#![allow(dead_code, unused_imports)]
use std::process::Command;

fn main() {
    let audio_args = ["--extract-audio", "--audio-format", "mp3"];
    let video_url = "kbNdx0yqbZE"; // temp video link

    let output = Command::new("yt-dlp")
        .arg("--ignore-config") // ignore system config
        .args(audio_args) // extract only audio
        .arg(video_url)
        .output()
        .expect("yt-dlp command failed.");

    println!("{:?}", output);
}
