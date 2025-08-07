#![allow(dead_code, unused_imports)]
use clap::Parser;
use dirs;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::process::{Command, Output};
use std::{
    fs::{self, File},
    io::{self, ErrorKind, Read, Write},
    path,
    str,
};

use prism::playlist;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Add a playlist to the tracking list
    #[arg(short = 'a', long = "add")]
    add: Option<String>,

    /// Remove a playlist from the tracking list
    #[arg(short = 'r', long = "remove")]
    remove: Option<String>,

    /// Download all tracked playlists
    #[arg(short = 'd', long = "download")]
    download_playlists: bool,

    /// List tracked playlists
    #[arg(short = 'l', long = "list", conflicts_with = "add")]
    list_playlists: bool,

    /// Check playlist health, downloading missing videos or
    /// archiving unavailable videos
    #[arg(short = 'c', long = "check", conflicts_with = "add")]
    check_playlists: bool,
}

const PROJECT_DIR: &str = "prism";

fn main() {
    let mut playlists: HashMap<String, String> = HashMap::new();
    let config_path = dirs::config_dir().unwrap().join(PROJECT_DIR);
    let music_path = dirs::audio_dir().unwrap().join(PROJECT_DIR);
    load(&mut playlists, &config_path.join("tracking"));

    let cli = Cli::parse();

    if let Some(url) = cli.add {
        playlist::add_playlist(&mut playlists, url);
    }

    if let Some(url) = cli.remove {
        playlist::remove_playlist(&mut playlists, url);
    }

    if cli.download_playlists {
        playlist::download_playlists(&playlists, &music_path);
    }

    if cli.list_playlists {
        for (url, title) in playlists.iter() {
            println!("{title} [{url}]");
        }
    }

    if cli.check_playlists {
        for (url, title) in &playlists {
            println!("Checking {}", title);
            playlist::check_playlist(url, &music_path.join(title));
        }
    }

    save(&playlists, &config_path.join("tracking"));
}

fn save(playlists: &HashMap<String, String>, file_path: impl AsRef<path::Path>) {
    let file_path = file_path.as_ref();
    let config_dir = file_path.parent().unwrap();

    fs::create_dir_all(config_dir).unwrap();

    let mut file = File::create(file_path).expect("Problem creating tracking file");

    if playlists.is_empty() { return };

    let mut pairs = String::new();
    for (url, title) in playlists.iter() {
        pairs.push_str(&*format!("{url}={title}\n"));
    }

    file.write_all(pairs.as_bytes()).expect("Problem saving to tracking file");
}

fn load(playlists: &mut HashMap<String, String>, file_path: impl AsRef<path::Path>) {
    let file_path = file_path.as_ref();
    let config_dir = file_path.parent().unwrap();

    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => return,
            other_err => panic!("Problem loading tracking file: {other_err:?}"),
        },
    };

    let mut s = String::new();
    if let Err(err) = file.read_to_string(&mut s) {
        if err.kind() == ErrorKind::IsADirectory {
            eprintln!("Conflicting directory named \"tracking\" in {}", config_dir.display());
        } else {
            eprintln!("Problem loading tracking file to string: {err:?}");
        }
    };

    for line in s.lines() {
        if let Some((url, title)) = line.split_once('=') {
            playlists.insert(url.to_string(), title.to_string());
        } else {
            println!("WARN: Line in tracking file failed to parse and was removed");
        }
    }
}