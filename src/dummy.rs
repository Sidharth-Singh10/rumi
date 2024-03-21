// use rodio::{Decoder, OutputStream, Sink};
// use std::fs::File;
// use std::io::BufReader;

// pub fn dummy() {
//     let (_stream, stream_handle) = OutputStream::try_default().unwrap();
//     let sink = Sink::try_new(&stream_handle).unwrap();
        
//     let file = BufReader::new(
//         File::open("music/Charlie Puth - Attention [Official Video] (320 kbps).mp3").unwrap(),
//     );
//     // Decode that sound file into a source
//     let source = Decoder::new(file).unwrap();
//     sink.append(source);

//     sink.sleep_until_end();
// }
