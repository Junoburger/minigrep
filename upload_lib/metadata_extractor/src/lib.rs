// use std::fs::File;
use std::io::Cursor;
use wasm_bindgen::prelude::*;

struct MetaData {
    // author::copyright
    copyright: String,
    // tags
    // title
    // createdISO8601: publiation_date
    // description
    description: String, // make (camera)
                         // model (camera)
                         // flash_found (camera)
                         // resolution: dpi (if dots per inch is 0 set it to 72)
                         // width
                         // height
                         // gps
}

pub fn handle_metadata() -> std::io::Result<()> {
    #[wasm_bindgen]
    pub fn get_metadata_from_image_blob(vector: Vec<u8>) {
        let mut file = Cursor::new(vector);

        let exifreader = exif::Reader::new();
        let exif = exifreader.read_from_container(&mut file).unwrap();

        match exif.get_field(exif::Tag::Copyright, exif::In::PRIMARY) {
            Some(copyright) => set_copy_right(copyright.display_value().to_string().as_str()),
            None => println!("No copyright tag found"),
        }

        // let latitude = exif.get_field(exif::Tag::GPSLatitude, exif::In::PRIMARY);
        // let longitude = exif.get_field(exif::Tag::GPSLongitude, exif::In::PRIMARY);

        for field in exif.fields() {
            // if field.tag == exif::Tag::GPSLatitude || field.tag == exif::Tag::GPSLongitude {
            log(&format!(
                "{:?} {:?} {}",
                field.tag,
                field.ifd_num,
                field.display_value().with_unit(&exif),
            ));
            // }
        }

        // return latitude
        //     .unwrap()
        //     .display_value()
        //     .with_unit(&exif)
        //     .to_string();
    }

    fn set_copy_right(copyright: &str) {
        log(&format!("COPYRIGHT = , {:?}!", copyright));
        // return copyright;
    }

    // let file = File::open("./files/yellow.tif")?;
    // let mut bufreader = std::io::BufReader::new(&file);

    // let exifreader = exif::Reader::new();
    // let exif = exifreader.read_from_container(&mut bufreader).unwrap();
    // for field in exif.fields() {
    //     println!(
    //         "{} {} {}",
    //         field.tag,
    //         field.ifd_num,
    //         field.display_value().with_unit(&exif),
    //     );
    // }

    // match exif.get_field(exif::Tag::Copyright, exif::In::PRIMARY) {
    //     Some(copyright) => set_copy_right(copyright.display_value().to_string().as_str()),
    //     None => println!("No copyright tag found"),
    // }

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
