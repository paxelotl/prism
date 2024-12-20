#![allow(dead_code, unused_imports)]
use clap::Parser;
use dirs;
use std::collections::HashMap;
use std::process::{Command, Output};
use std::{
    fs::{self, File, OpenOptions},
    io::{ErrorKind, Read, Write},
    str,
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Adds a playlist to the tracking list
    #[arg(short = 'a', long = "add")]
    url: Option<String>,

    /// Download all tracked playlists
    #[arg(short = 'd', long = "download")]
    download_playlists: bool,

    /// List tracked playlists
    #[arg(short = 'l', long = "list", conflicts_with = "url")]
    list_playlists: bool,
}

const PROJECT_DIR: &str = "prism";

fn main() {
    let mut playlists: HashMap<String, String> = HashMap::new();
    let config_path = dirs::config_dir().unwrap().join(PROJECT_DIR);
    let music_path = dirs::audio_dir().unwrap().join(PROJECT_DIR);
    load(&mut playlists, &config_path.join("tracking"));

    let cli = Cli::parse();

    if let Some(url) = cli.url {
        track_playlist(&mut playlists, url);
    }

    if cli.download_playlists {
        download_playlists(&playlists, &music_path);
    }

    if cli.list_playlists {
        for (url, title) in playlists.iter() {
            println!("{title} [{url}]");
        }
    }

    save(&playlists, &config_path.join("tracking"));
}

fn download_playlists(playlists: &HashMap<String, String>, download_path: &std::path::PathBuf) {
    for (url, title) in playlists.iter() {
        let path = download_path.join(title);
        println!("{path:?}");
        let audio_args = ["--extract-audio", "--audio-format", "opus"];

        Command::new("yt-dlp")
            .arg("--ignore-config") // ignore system config
            .args(audio_args) // extract only audio
            .arg("--path") // yt-dlp will just create the path if it's not there
            .arg(path)
            .arg(url)
            .output()
            .expect("Downloading playlists failed");
    }
}

fn track_playlist(playlists: &mut HashMap<String, String>, playlist_url: String) {
    // check if playlist already tracked
    if let Some(title) = playlists.get(&playlist_url) {
        println!("INFO: Playlist \"{title}\" is already tracked");
    } else {
        let playlist_title = get_playlist_title(playlist_url.clone());
        playlists.insert(playlist_url, playlist_title);
    }
}

fn save(playlists: &HashMap<String, String>, file_path: impl AsRef<std::path::Path>) {
    let mut pairs = String::new();
    for (url, title) in playlists.iter() {
        pairs.push_str(&*format!("{url}={title}\n"));
    }

    let mut file = File::create(file_path).expect("file creation failed");
    file.write_all(pairs.as_bytes())
        .expect("write to file failed");
}

fn load(playlists: &mut HashMap<String, String>, file_path: impl AsRef<std::path::Path>) {
    let mut file = match OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)
    {
        Ok(file) => file,
        Err(error) => panic!("Problem opening or creating \"tracking\" file: {error:?}"),
    };

    let mut s = String::new();
    file.read_to_string(&mut s)
        .expect("Problem reading \"tracking\" file to string");
    for line in s.lines() {
        if let Some((url, title)) = line.split_once('=') {
            playlists.insert(url.to_string(), title.to_string());
        } else {
            println!("WARN: Line in \"tracking\" file failed to parse and was removed");
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
