use id3::Tag;
// use ratatui_image::FilterType;
use std::fs::File;
use std::io::prelude::*;

// use ratatui_image::picker::Picker;
use ratatui_image::picker::Picker;
use ratatui_image::protocol::StatefulProtocol;

pub fn extract_image(mp3_path: String,output_path: String,) -> Result<(), Box<dyn std::error::Error>>
{
    let tag = Tag::read_from_path(mp3_path)?;
    if let Some(picture) = tag.pictures().next() {
        let mut file = File::create(output_path)?;
        file.write_all(&picture.data)?;
    }
    Ok(())
}

pub fn display_image(input_path: String) -> Box<dyn StatefulProtocol> {
    let mut picker = Picker::from_termios().unwrap();

    picker.guess_protocol();
    let dyn_img = image::io::Reader::open(input_path)
        .unwrap()
        .decode()
        .unwrap();

   
    let image = picker.new_resize_protocol(dyn_img);
    image
}


