use rodio::*;
use std::fs::File;
use std::io::BufReader;

pub fn play_audio(pathname: &str) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let file = File::open(pathname).unwrap();
    let source = Decoder::new(BufReader::new(file)).unwrap();

    let sink = Sink::try_new(&stream_handle).unwrap();
    sink.append(source);

    sink.sleep_until_end();
}
