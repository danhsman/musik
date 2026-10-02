use rodio::Player;
use rodio::source::{self, Mix};
use rodio::{Decoder, MixerDeviceSink, source::Source};
use std::fs::File;
use std::io::BufReader;

struct music_player {
    _device: MixerDeviceSink,
    player: Player,
    queue: Vec<String>,
    current: usize,
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

    fn add_track(&mut self, path: &str) {
        self.queue.push(path.to_string());
    }

    fn next(&mut self) {
        if self.current + 1 < self.queue.len() {
            self.current += 1;
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

    fn is_finished(&self) -> bool {
        self.player.empty()
    }
}

pub fn main() -> () {
    let mut mp = music_player::new();
    mp.add_track("examples/HeliMAn.mp3");
    mp.play_current();
    mp.player.sleep_until_end();
}
