use std::path::PathBuf;

pub mod parser;

pub fn form_img(path: &mut PathBuf, w: &u32, h: &u32, args: Option<u8>) -> () {
    path.set_extension("png");

    let col = match args {
        Some(t) => t,
        None => 255,
    };

    let imgbuff = image::ImageBuffer::from_fn(*w, *h, |_x, _y| {
        image::Luma([col as u8])
    });
    let path_str = path.to_str();

    match imgbuff.save(&*path) {
        Ok(_t) => {println!("Wrote new image at {:?}", path_str.unwrap())},
        Err(_t) => {println!("Error writing to image file at {:?}", path_str.unwrap())},
    }
}