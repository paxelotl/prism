#![allow(dead_code, unused_imports)]
use clap::Parser;
use std::collections::HashMap;
use std::process::{Command, Output};
use std::{
    fs::File,
    io::{ErrorKind, Read, Write},
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
    let mut playlists: HashMap<String, String> = HashMap::new();
    load(&mut playlists);

    if let Some(url) = cli.url {
        track_playlist(&mut playlists, url);
    }

    println!("printing playlists: {}", cli.list_playlists);

    save(&playlists);
}

fn track_playlist(playlists: &mut HashMap<String, String>, playlist_url: String) {
    // TODO: check if playlist already tracked
    let playlist_title = get_playlist_title(playlist_url.clone());

    let mut file = File::create("tracking").expect("file creation failed");
    file.write_all(format!("{}={}", playlist_title, playlist_url).as_bytes())
        .expect("write to file failed");
}

fn save(playlists: &HashMap<String, String>) {
    let mut pairs = String::new();
    for (name, url) in playlists.iter() {
        pairs.push_str(&*format!("{name}={url}"));
    }

    let mut file = File::create("tracking").expect("file creation failed");
    file.write_all(pairs.as_bytes())
        .expect("write to file failed");
}

fn load(playlists: &mut HashMap<String, String>) {
    let mut file = match File::open("tracking") {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("tracking") {
                Ok(file) => file,
                Err(error) => panic!("Problem creating the file: {error:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };

    let mut s = String::new();
    file.read_to_string(&mut s)
        .expect("Problem reading \"tracking\" file to string: ");
    for line in s.lines() {
        if let Some((name, url)) = line.split_once('=') {
            playlists.insert(name.to_string(), url.to_string());
        }
    }
}

fn get_playlist_title(playlist_url: String) -> String {
    let title_args = ["--flat-playlist", "--print", "playlist_title", "-I", "1:1"];

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
