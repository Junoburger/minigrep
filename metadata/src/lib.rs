use exif::Reader;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn test_metadata(file: &str) -> Result<String, String> {
    let mut result = String::new();

    let file = std::fs::File::open(&file).unwrap();
    let reader = Reader::new();
    let exif = reader
        .read_from_container(&mut std::io::BufReader::new(&file))
        .unwrap();

    result.push_str(&format!("{}", &exif.little_endian()));

    // for f in exif.fields() {
    //     result.push_str(&format!("{} {} {}", f.tag, f.ifd_num, f.display_value()));
    // }
    // test_metadata("test.jpg").unwrap();
    Ok(result)
}

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
