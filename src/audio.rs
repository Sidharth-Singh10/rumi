use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::fs::{self, File};
use std::io::BufReader;
use std::path::PathBuf;

pub struct Controls {
    pub sink: Sink,
    output_stream: OutputStream,
    stream_handle: OutputStreamHandle,
    pub playlistz: Vec<PathBuf>,
    pub songlist: Vec<String>,
}

impl Controls 
{
    pub fn new() -> Controls 
    {
        let mut playlistz: Vec<PathBuf> = Vec::new();
        let mut songlist: Vec<String> = Vec::new();

        let (_stream, stream_handle) = OutputStream::try_default().unwrap();

        let sink = Sink::try_new(&stream_handle).unwrap();

        let folder_path = PathBuf::from("music");
        for entry in fs::read_dir(folder_path).unwrap() 
        {
            let path = entry.unwrap().path();
            if path.is_file() 
            {
                if let Some(filename) = path.file_name() 
                {
                    if let Some(filename_str) = filename.to_str() 
                    {
                        songlist.push(filename_str.to_string());
                    }
                playlistz.push(path);
                }
            }
        }

        Controls {
            sink,
            output_stream: _stream,
            stream_handle,
            playlistz,
            songlist,
        }
    }


    pub fn get_source(path: PathBuf) -> Decoder<BufReader<File>> {
        let file = std::fs::File::open(&path).unwrap();
        let source: Decoder<BufReader<File>> = Decoder::new(BufReader::new(file)).unwrap();
        source
    }

}
