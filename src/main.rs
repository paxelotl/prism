#![allow(dead_code, unused_imports)]
use clap::Parser;
use std::collections::HashMap;
use std::process::{Command, Output};
use std::{
    fs,
    io::{Read, Write},
    str,
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Adds a playlist to the tracking list
    #[arg(short = 'a', long = "add")]
    url: Option<String>,

    /// List tracked playlists
    #[arg(short = 'l', long = "list", conflicts_with = "url")]
    list_playlists: bool,
}

fn main() {
    let cli = Cli::parse();

    if let Some(playlist_url) = cli.url {
        println!("{playlist_url}");
    } else {
        println!("add command not used");
    }

    println!("printing playlists: {}", cli.list_playlists);
}

fn load_playlists() -> HashMap<String, String> {
    let mut playlists: HashMap<String, String> = HashMap::new();
    let mut file = fs::File::open("tracking").expect("failed to open file");
    let mut s = String::new();
    file.read_to_string(&mut s)
        .expect("failed to read file to string");
    for line in s.lines() {
        if let Some((left, right)) = line.split_once('=') {
            playlists.insert(left.to_string(), right.to_string());
        }
    }

    playlists
}

fn track_playlist(playlist_url: String) {
    // TODO: check if playlist already tracked
    let playlist_title = get_playlist_title(playlist_url.clone());

    let mut file = fs::File::create("tracking").expect("file creation failed");
    file.write_all(format!("{}={}", playlist_title, playlist_url).as_bytes())
        .expect("write to file failed");
}

fn get_playlist_title(playlist_url: String) -> String {
    let title_args = ["--print", "playlist_title", "--skip-download", "-I", "1:1"];
    let output = Command::new("yt-dlp")
        .arg("--ignore-config")
        .args(title_args)
        .arg(playlist_url)
        .output()
        .expect("failed getting playlist title");

    let title = str::from_utf8(&output.stdout)
        .expect("failed utf8 conversion")
        .replace("\\n", "\n");

    title.trim().to_string()
}

fn vid() {
    let video_url = "kbNdx0yqbZE"; // temp video link
    let audio_args = ["--extract-audio", "--audio-format", "mp3"];

    let output = Command::new("yt-dlp")
        .arg("--ignore-config") // ignore system config
        .args(audio_args) // extract only audio
        .arg(video_url)
        .output()
        .expect("yt-dlp command failed");

    let stdout = str::from_utf8(&output.stdout)
        .expect("failed utf8 conversion")
        .replace("\\n", "\n");

    println!("{}", stdout.trim());
}
