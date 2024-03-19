use rodio::{Decoder, OutputStream, Sink, OutputStreamHandle};
use std::fs;
use std::io::BufReader;
use std::path::PathBuf;

pub struct Controls {
    pub sink: Sink,
    output_stream: OutputStream,
    stream_handle: OutputStreamHandle
}
impl Controls {
    pub fn new() -> Controls {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();

        let sink = Sink::try_new(&stream_handle).unwrap();


        let file = BufReader::new(
            std::fs::File::open("music/Charlie Puth - Attention [Official Video] (320 kbps).mp3")
                .unwrap(),
        );
        let source = Decoder::new(file).unwrap();

        sink.append(source);
        sink.pause();

        Controls{   
            sink,
            output_stream: _stream,
            stream_handle: stream_handle
        }
    }


    fn play(&self) {
        self.sink.play();
    }   
}
