use rodio::Player;
use rodio::microphone::Input;
use rodio::source::{self, Mix};
use rodio::{Decoder, MixerDeviceSink, source::Source};
use std::fs::{self, File};
use std::io::BufReader;
use std::io::{self, Write};
use std::path::PathBuf;

struct music_player {
    _device: MixerDeviceSink,
    player: Player,
    queue: Vec<PathBuf>,
    current: usize,
}

fn list_tracks(dir: &str) -> Vec<PathBuf> {
    let mut tracks: Vec<PathBuf> = fs::read_dir(dir)
        .expect("can't read directory")
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|p| {
            p.extension()
                .and_then(|e| e.to_str())
                .map(|e| matches!(e.to_lowercase().as_str(), "mp3" | "flac" | "wav" | "ogg"))
                .unwrap_or(false)
        })
        .collect();
    tracks.sort();
    tracks
}

impl music_player {
    fn new() -> Self {
        let device =
            rodio::DeviceSinkBuilder::open_default_sink().expect("open default audio stream");

        let player = Player::connect_new(device.mixer());
        music_player {
            _device: (device),
            player,
            queue: (Vec::new()),
            current: (0),
        }
    }
    fn play_current(&mut self) {
        let file = BufReader::new(File::open(&self.queue[self.current]).unwrap());
        let source = Decoder::new(file).unwrap();
        self.player.stop();
        self.player.append(source);
        self.player.play();
    }

    fn add_track(&mut self, path: PathBuf) {
        self.queue.push(path);
    }

    fn play_index(&mut self, index: usize) {
        if index < self.queue.len() {
            self.current = index;
            self.play_current();
        }
    }

    fn play_pause(&mut self) {
        if self.player.empty() {
            self.play_current();
        } else if self.player.is_paused() {
            self.player.play();
        } else {
            self.player.pause();
        }
    }

    //fn repeat(&mut self) { }

    fn is_finished(&self) -> bool {
        self.player.empty()
    }
}

fn choose(tracks: &[PathBuf]) -> Option<usize> {
    for (i, track) in tracks.iter().enumerate() {
        println!("{}: {}", i + 1, track.file_name()?.to_string_lossy());
    }

    println!("Pih song: ");
    io::stdout().flush().ok()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input).ok()?;

    let n: usize = input.trim().parse().ok()?;
    n.checked_sub(1).filter(|&i| i < tracks.len())
}

pub fn main() -> () {
    let tracks = list_tracks("examples");

    if tracks.is_empty() {
        println!("No tracks found");
        return;
    }

    let Some(choice) = choose(&tracks) else {
        println!("Invalid selection.");
        return;
    };

    let mut mp = music_player::new();
    for track in tracks {
        mp.add_track(track);
    }
    mp.play_index(choice);
    mp.player.sleep_until_end();
}
