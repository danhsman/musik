use rodio::source;
use rodio::{Decoder, MixerDeviceSink, source::Source};
use std::fs::File;
use std::io::BufReader;

pub fn main() -> () {
    play();
}

fn play() -> () {
    let sink_handle =
        rodio::DeviceSinkBuilder::open_default_sink().expect("open default audio stream");

    let file = BufReader::new(File::open("examples/HeliMAn.mp3").unwrap());

    let player = rodio::play(&sink_handle.mixer(), file).unwrap();

    player.sleep_until_end();
}
