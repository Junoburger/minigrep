use std::fs::File;
use wasm_bindgen::prelude::*;

// author::copyright
// tags
// title
// createdISO8601: publiation_date
// description
// make (camera)
// model (camera)
// flash_found (camera)
// resolution: dpi (if dots per inch is 0 set it to 72)
// width
// height
// gps
//
pub fn handle_metadata() -> std::io::Result<()> {
    #[wasm_bindgen]
    pub fn get_file_data(file: &str) {
        log(&format!("Hello, {:?}!", file));
    }

    let file = File::open("./files/yellow.tif")?;
    let mut bufreader = std::io::BufReader::new(&file);

    let exifreader = exif::Reader::new();
    let exif = exifreader.read_from_container(&mut bufreader).unwrap();
    // for field in exif.fields() {
    //     println!(
    //         "{} {} {}",
    //         field.tag,
    //         field.ifd_num,
    //         field.display_value().with_unit(&exif),
    //     );
    // }

    fn set_copy_right(copyright: &str) {
        println!("COPY {}", copyright);
    }

    match exif.get_field(exif::Tag::Copyright, exif::In::PRIMARY) {
        Some(copyright) => set_copy_right(copyright.display_value().to_string().as_str()),
        None => println!("No copyright tag found"),
    }

    Ok(())
}

// Javascript Debug tools
#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log_many(a: &str, b: &str);

    // ALERT
    fn alert(s: &str);

}
