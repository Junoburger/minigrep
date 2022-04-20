// use exif::Reader;
use image::{DynamicImage, ImageFormat};
use std::io;
use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
// pub fn test_metadata(file: &str) -> Result<String, String> {
//     let mut result = String::new();

//     let file = std::fs::File::open(&file).unwrap();
//     let reader = Reader::new();
//     let exif = reader
//         .read_from_container(&mut std::io::BufReader::new(&file))
//         .unwrap();

//     result.push_str(&format!("{}", &exif.little_endian()));

//     for f in exif.fields() {
//         result.push_str(&format!("{} {} {}", f.tag, f.ifd_num, f.display_value()));
//     }
//     // test_metadata("test.jpg").unwrap();
//     // let emoji = "\u{1f600}";
//     Ok(result.to_string())
// }

// #[wasm_bindgen]
// pub fn test_meta() -> Result<String, String> {
//     use std::fs;

//     let metadata = fs::metadata("some.txt").unwrap();
//     Ok(format!("{:?}", metadata.file_type()))
// }

// ------------------------------------------------------------------------------------------------
#[wasm_bindgen]
pub struct Image {
    image: DynamicImage,
    // width: u32,
    // height: u32,
    extension: image::ImageFormat,
}

#[wasm_bindgen]
impl Image {
    pub fn new(image_blob: Vec<u8>) -> Image {
        let reader = image::io::Reader::new(io::Cursor::new(image_blob))
            .with_guessed_format()
            .expect("Cursor io never fails");

        let extension = match reader.format() {
            Some(extension) => extension,
            None => panic!("Can't guess image extension"),
        };

        // let width;
        // let height;
        let image = match reader.decode() {
            Ok(image) => {
                // let (w, h) = image.dimensions();
                // width = w;
                // height = h;
                image
            }
            Err(error) => {
                panic!("Can't get image: {}", error)
            }
        };

        Image {
            image,
            // width,
            // height,
            extension,
        }
    }

    pub fn get_image_blob(&self) -> Vec<u8> {
        match self.extension {
            // x if x == ImageFormat::Png => self.image.to_rgba8().to_vec(),
            _ => self.image.to_rgb8().to_vec(),
        }
    }
}

// ------------------------------------------------------------------------------------------------

// #[wasm_bindgen]
// pub fn get_file_metadata(file: &str) -> Result<String, String> {
//     let file = std::fs::File::open(file).unwrap();
//     let exif = Reader::new()
//         .read_from_container(&mut std::io::BufReader::new(&file))
//         .unwrap();
//     let mut result = String::new();
//     for f in exif.fields() {
//         result.push_str(&format!("{} {} {}", f.tag, f.ifd_num, f.display_value()));
//     }
//     Ok(result)
// }

// Import the `window.alert` function from the Web.
// #[wasm_bindgen]
// extern "C" {
// fn alert(s: &str);
// }

// Export a `greet` function from Rust to JavaScript, that alerts a
// hello message.
// #[wasm_bindgen]
// pub fn greet(name: &str) {
// alert(&format!("Hello, {}!", name));
// }
