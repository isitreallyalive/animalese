use std::io::Cursor;

use include_dir::File;
use rodio::{Decoder, OutputStream, OutputStreamBuilder, Sink, StreamError};

pub const VOICE: include_dir::Dir = include_dir::include_dir!("assets/voices/ac/jp/boy");

pub struct AudioPlayer {
    stream: OutputStream,
    sinks: Vec<Sink>,
}

impl AudioPlayer {
    pub fn new() -> Result<Self, StreamError> {
        Ok(Self {
            stream: OutputStreamBuilder::open_default_stream()?,
            sinks: Vec::new(),
        })
    }

    pub fn play(&mut self, sounds: Vec<&File>) {
        // remove empty sinks
        self.sinks.retain(|sink| !sink.empty());

        // play the sounds in a new sink
        let sink = Sink::connect_new(self.stream.mixer());
        sounds
            .iter()
            .map(|sound| Decoder::new_vorbis(Cursor::new(sound.contents().to_vec())))
            .filter_map(|decoder| decoder.ok())
            .for_each(|sound| sink.append(sound));
        self.sinks.push(sink);
    }
}
