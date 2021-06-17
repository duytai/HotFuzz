extern crate image;
use image::GenericImageView;
use std::env;
use std::io;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        let mut f = File::open(&args[1]).unwrap();
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer).unwrap();
        let img = image::load_from_memory_with_format(&buffer, image::ImageFormat::Jpeg).unwrap();
        // The dimensions method returns the images width and height.
        println!("dimensions {:?}", img.dimensions());
        // The color method returns the image's `ColorType`.
        println!("{:?}", img.color());
    }
}
