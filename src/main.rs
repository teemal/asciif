use image::gif::Decoder;
use image::AnimationDecoder;
use image::ImageFormat::JPEG;

use std::fs::File;

fn decode() {
    // Decode a gif into frames
    let file_in = File::open("devitobb.gif").expect("Could not open file");
    let mut decoder = Decoder::new(file_in).expect("Could not create decoder");

    let frames = decoder.into_frames();
    let frames = frames.collect_frames().expect("error decoding gif");

    let mut i = 0;
    for f in frames {
        f.into_buffer().save_with_format(
            format!("dist/media/{}.jpg", i),
            JPEG
        ).expect("Could not save frame");
        i += 1;
    }
}

fn main() {
    decode();
}
